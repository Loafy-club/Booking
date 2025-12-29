use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};
use loafy_db::{queries::{bookings, subscriptions, ticket_transactions}, PgPool};
use stripe::{Event, EventObject, EventType, Webhook};
use uuid::Uuid;

/// Number of tickets granted per subscription purchase/renewal
const SUBSCRIPTION_TICKETS: i32 = 10;

/// Handle Stripe webhook event with signature verification
pub async fn handle_stripe_webhook(
    payload: &str,
    signature: &str,
    webhook_secret: &str,
    pool: &PgPool,
) -> Result<()> {
    // Verify webhook signature and construct event
    let event = Webhook::construct_event(payload, signature, webhook_secret)
        .map_err(|e| anyhow!("Webhook signature verification failed: {}", e))?;

    tracing::info!("Received Stripe webhook: {:?} ({})", event.type_, event.id);

    // Route to appropriate handler based on event type
    match event.type_ {
        // Payment Intent events (for bookings)
        EventType::PaymentIntentSucceeded => {
            handle_payment_succeeded(&event, pool).await?;
        }
        EventType::PaymentIntentPaymentFailed => {
            handle_payment_failed(&event).await?;
        }
        EventType::PaymentIntentCanceled => {
            handle_payment_canceled(&event).await?;
        }
        // Subscription events
        EventType::CheckoutSessionCompleted => {
            handle_checkout_completed(&event, pool).await?;
        }
        EventType::InvoicePaid => {
            handle_invoice_paid(&event, pool).await?;
        }
        EventType::InvoicePaymentFailed => {
            handle_invoice_payment_failed(&event, pool).await?;
        }
        EventType::CustomerSubscriptionUpdated => {
            handle_subscription_updated(&event, pool).await?;
        }
        EventType::CustomerSubscriptionDeleted => {
            handle_subscription_deleted(&event, pool).await?;
        }
        _ => {
            tracing::debug!("Unhandled webhook event type: {:?}", event.type_);
        }
    }

    Ok(())
}

/// Extract booking UUID from PaymentIntent metadata
fn extract_booking_id(payment_intent: &stripe::PaymentIntent) -> Result<Uuid> {
    let metadata = &payment_intent.metadata;

    let booking_id_str = metadata
        .get("booking_id")
        .ok_or_else(|| anyhow!("No booking_id in PaymentIntent metadata"))?;

    Uuid::parse_str(booking_id_str)
        .map_err(|e| anyhow!("Invalid booking UUID in metadata: {}", e))
}

/// Handle successful payment
async fn handle_payment_succeeded(event: &Event, pool: &PgPool) -> Result<()> {
    let payment_intent = match &event.data.object {
        EventObject::PaymentIntent(pi) => pi,
        _ => return Err(anyhow!("Expected PaymentIntent in event data")),
    };

    let booking_uuid = extract_booking_id(payment_intent)?;
    let payment_intent_id = payment_intent.id.as_str();

    // Update booking to confirmed status
    bookings::update_payment_status(pool, booking_uuid, "confirmed", Some(payment_intent_id))
        .await
        .map_err(|e| anyhow!("Failed to update booking payment status: {}", e))?;

    tracing::info!(
        "Payment succeeded for booking {} (PaymentIntent: {})",
        booking_uuid,
        payment_intent_id
    );

    Ok(())
}

/// Handle failed payment
async fn handle_payment_failed(event: &Event) -> Result<()> {
    let payment_intent = match &event.data.object {
        EventObject::PaymentIntent(pi) => pi,
        _ => return Err(anyhow!("Expected PaymentIntent in event data")),
    };

    let booking_uuid = extract_booking_id(payment_intent)?;

    // Log the failure but don't cancel - let the deadline job handle it
    // This allows users to retry payment before deadline
    tracing::warn!(
        "Payment failed for booking {} (PaymentIntent: {}). User can retry before deadline.",
        booking_uuid,
        payment_intent.id
    );

    Ok(())
}

/// Handle canceled payment intent
async fn handle_payment_canceled(event: &Event) -> Result<()> {
    let payment_intent = match &event.data.object {
        EventObject::PaymentIntent(pi) => pi,
        _ => return Err(anyhow!("Expected PaymentIntent in event data")),
    };

    let booking_uuid = extract_booking_id(payment_intent)?;

    tracing::info!(
        "Payment intent canceled for booking {} (PaymentIntent: {})",
        booking_uuid,
        payment_intent.id
    );

    Ok(())
}

// ============================================================================
// Subscription Event Handlers
// ============================================================================

/// Convert Stripe timestamp to DateTime<Utc>
fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(timestamp, 0).single().unwrap_or_else(Utc::now)
}

/// Extract user_id from metadata
fn extract_user_id_from_metadata(metadata: &Option<std::collections::HashMap<String, String>>) -> Result<Uuid> {
    let metadata = metadata.as_ref().ok_or_else(|| anyhow!("No metadata found"))?;
    let user_id_str = metadata
        .get("user_id")
        .ok_or_else(|| anyhow!("No user_id in metadata"))?;
    Uuid::parse_str(user_id_str).map_err(|e| anyhow!("Invalid user UUID in metadata: {}", e))
}

/// Handle checkout.session.completed - first subscription purchase
/// This is called when a user completes the Stripe Checkout flow
async fn handle_checkout_completed(event: &Event, pool: &PgPool) -> Result<()> {
    let session = match &event.data.object {
        EventObject::CheckoutSession(s) => s,
        _ => return Err(anyhow!("Expected CheckoutSession in event data")),
    };

    // Only process subscription mode sessions
    if session.mode != stripe::CheckoutSessionMode::Subscription {
        tracing::debug!("Checkout session is not subscription mode, skipping");
        return Ok(());
    }

    let user_id = extract_user_id_from_metadata(&session.metadata)?;

    let subscription_id = session
        .subscription
        .as_ref()
        .and_then(|s| match s {
            stripe::Expandable::Id(id) => Some(id.as_str().to_string()),
            stripe::Expandable::Object(sub) => Some(sub.id.as_str().to_string()),
        })
        .ok_or_else(|| anyhow!("No subscription ID in checkout session"))?;

    let customer_id = session
        .customer
        .as_ref()
        .and_then(|c| match c {
            stripe::Expandable::Id(id) => Some(id.as_str().to_string()),
            stripe::Expandable::Object(cust) => Some(cust.id.as_str().to_string()),
        })
        .ok_or_else(|| anyhow!("No customer ID in checkout session"))?;

    // Check if subscription already exists (idempotency)
    if let Some(existing) = subscriptions::find_by_stripe_subscription_id(pool, &subscription_id).await? {
        tracing::info!(
            "Subscription {} already exists for user {}, skipping creation",
            subscription_id,
            existing.user_id
        );
        return Ok(());
    }

    tracing::info!(
        "Checkout completed for user {}, subscription: {}, customer: {}",
        user_id,
        subscription_id,
        customer_id
    );

    // Note: Ticket allocation happens in invoice.paid handler
    // This handler just logs the completion
    Ok(())
}

/// Handle invoice.paid - initial payment and renewals
/// This is the main handler for allocating tickets
async fn handle_invoice_paid(event: &Event, pool: &PgPool) -> Result<()> {
    let invoice = match &event.data.object {
        EventObject::Invoice(inv) => inv,
        _ => return Err(anyhow!("Expected Invoice in event data")),
    };

    // Only process subscription invoices
    let subscription_id = match &invoice.subscription {
        Some(stripe::Expandable::Id(id)) => id.as_str().to_string(),
        Some(stripe::Expandable::Object(sub)) => sub.id.as_str().to_string(),
        None => {
            tracing::debug!("Invoice has no subscription, skipping");
            return Ok(());
        }
    };

    let customer_id = match &invoice.customer {
        Some(stripe::Expandable::Id(id)) => id.as_str().to_string(),
        Some(stripe::Expandable::Object(cust)) => cust.id.as_str().to_string(),
        None => {
            tracing::warn!("Invoice has no customer ID");
            return Err(anyhow!("Invoice has no customer ID"));
        }
    };

    // Get period dates from invoice lines
    let (period_start, period_end) = invoice
        .lines
        .as_ref()
        .and_then(|lines| lines.data.first())
        .and_then(|line| line.period.as_ref())
        .and_then(|period| {
            match (period.start, period.end) {
                (Some(start), Some(end)) => Some((
                    timestamp_to_datetime(start),
                    timestamp_to_datetime(end),
                )),
                _ => None,
            }
        })
        .unwrap_or_else(|| {
            // Default to 3 months if period not available
            let now = Utc::now();
            (now, now + chrono::Duration::days(90))
        });

    // Check if we have an existing subscription
    let existing_sub = subscriptions::find_by_stripe_subscription_id(pool, &subscription_id).await?;

    if let Some(sub) = existing_sub {
        // Check if this is a duplicate event (same period_end means we already processed this invoice)
        if let Some(existing_period_end) = sub.current_period_end {
            // If period_end matches within a few seconds, this is a duplicate event
            let diff = (period_end - existing_period_end).num_seconds().abs();
            if diff < 60 {
                tracing::info!(
                    "Duplicate invoice.paid event for subscription {}, period_end matches existing (diff: {}s). Skipping.",
                    subscription_id,
                    diff
                );
                return Ok(());
            }
        }

        // This is a true renewal - period_end has changed
        tracing::info!(
            "Processing subscription renewal for user {}, subscription: {}",
            sub.user_id,
            subscription_id
        );

        let updated_sub = subscriptions::renew_subscription(
            pool,
            sub.id,
            SUBSCRIPTION_TICKETS,
            period_end,
        )
        .await?;

        // Record ticket transaction
        ticket_transactions::create_with_pool(
            pool,
            sub.user_id,
            Some(sub.id),
            None,
            "subscription_grant",
            SUBSCRIPTION_TICKETS,
            updated_sub.tickets_remaining,
            Some("Subscription renewal"),
            None,
        )
        .await?;

        tracing::info!(
            "Renewed subscription for user {}: +{} tickets, new balance: {}",
            sub.user_id,
            SUBSCRIPTION_TICKETS,
            updated_sub.tickets_remaining
        );
    } else {
        // This is a new subscription - need to get user_id from customer metadata
        // or from the subscription metadata

        // Try to get user_id from invoice subscription_details metadata
        let user_id = invoice
            .subscription_details
            .as_ref()
            .and_then(|details| details.metadata.as_ref())
            .and_then(|meta| meta.get("user_id"))
            .and_then(|id| Uuid::parse_str(id).ok());

        let user_id = match user_id {
            Some(id) => id,
            None => {
                tracing::warn!(
                    "No user_id found in invoice metadata for subscription {}, trying customer lookup",
                    subscription_id
                );
                // Try to find by customer_id
                if let Some(existing_by_customer) = subscriptions::find_by_stripe_customer_id(pool, &customer_id).await? {
                    existing_by_customer.user_id
                } else {
                    tracing::error!(
                        "Cannot determine user_id for new subscription {}, customer {}",
                        subscription_id,
                        customer_id
                    );
                    return Err(anyhow!("Cannot determine user_id for subscription"));
                }
            }
        };

        // Create new subscription
        let new_sub = subscriptions::create(
            pool,
            user_id,
            &subscription_id,
            &customer_id,
            SUBSCRIPTION_TICKETS,
            period_start,
            period_end,
        )
        .await?;

        // Record ticket transaction
        ticket_transactions::create_with_pool(
            pool,
            user_id,
            Some(new_sub.id),
            None,
            "subscription_grant",
            SUBSCRIPTION_TICKETS,
            SUBSCRIPTION_TICKETS,
            Some("Initial subscription purchase"),
            None,
        )
        .await?;

        tracing::info!(
            "Created subscription for user {}: {} tickets, period ends {}",
            user_id,
            SUBSCRIPTION_TICKETS,
            period_end
        );
    }

    Ok(())
}

/// Handle invoice.payment_failed - mark subscription as past_due
async fn handle_invoice_payment_failed(event: &Event, pool: &PgPool) -> Result<()> {
    let invoice = match &event.data.object {
        EventObject::Invoice(inv) => inv,
        _ => return Err(anyhow!("Expected Invoice in event data")),
    };

    let subscription_id = match &invoice.subscription {
        Some(stripe::Expandable::Id(id)) => id.as_str().to_string(),
        Some(stripe::Expandable::Object(sub)) => sub.id.as_str().to_string(),
        None => {
            tracing::debug!("Invoice has no subscription, skipping");
            return Ok(());
        }
    };

    // Update subscription status
    if let Some(_sub) = subscriptions::find_by_stripe_subscription_id(pool, &subscription_id).await? {
        subscriptions::update_from_stripe(
            pool,
            &subscription_id,
            "past_due",
            None,
            None,
            false,
        )
        .await?;

        tracing::warn!(
            "Payment failed for subscription {}, marked as past_due",
            subscription_id
        );
    }

    Ok(())
}

/// Handle customer.subscription.updated - sync status changes
async fn handle_subscription_updated(event: &Event, pool: &PgPool) -> Result<()> {
    let subscription = match &event.data.object {
        EventObject::Subscription(sub) => sub,
        _ => return Err(anyhow!("Expected Subscription in event data")),
    };

    let subscription_id = subscription.id.as_str();

    // Map Stripe status to our status
    let status = match subscription.status {
        stripe::SubscriptionStatus::Active => "active",
        stripe::SubscriptionStatus::PastDue => "past_due",
        stripe::SubscriptionStatus::Canceled => "cancelled",
        stripe::SubscriptionStatus::Unpaid => "past_due",
        stripe::SubscriptionStatus::Incomplete => "past_due",
        stripe::SubscriptionStatus::IncompleteExpired => "expired",
        stripe::SubscriptionStatus::Trialing => "active",
        stripe::SubscriptionStatus::Paused => "cancelled",
    };

    let period_start = Some(timestamp_to_datetime(subscription.current_period_start));
    let period_end = Some(timestamp_to_datetime(subscription.current_period_end));
    let cancel_at_period_end = subscription.cancel_at_period_end;

    // Check if subscription exists in our DB
    if subscriptions::find_by_stripe_subscription_id(pool, subscription_id).await?.is_some() {
        subscriptions::update_from_stripe(
            pool,
            subscription_id,
            status,
            period_start,
            period_end,
            cancel_at_period_end,
        )
        .await?;

        tracing::info!(
            "Updated subscription {} status to {}, cancel_at_period_end: {}",
            subscription_id,
            status,
            cancel_at_period_end
        );
    } else {
        tracing::debug!(
            "Subscription {} not found in database, skipping update",
            subscription_id
        );
    }

    Ok(())
}

/// Handle customer.subscription.deleted - mark as expired/cancelled
/// Note: We keep remaining tickets - user paid for them
async fn handle_subscription_deleted(event: &Event, pool: &PgPool) -> Result<()> {
    let subscription = match &event.data.object {
        EventObject::Subscription(sub) => sub,
        _ => return Err(anyhow!("Expected Subscription in event data")),
    };

    let subscription_id = subscription.id.as_str();

    // Mark as expired (not cancelled - they may have tickets remaining)
    if let Some(sub) = subscriptions::find_by_stripe_subscription_id(pool, subscription_id).await? {
        subscriptions::mark_expired(pool, subscription_id).await?;

        tracing::info!(
            "Subscription {} deleted/expired for user {}. Tickets remaining: {}",
            subscription_id,
            sub.user_id,
            sub.tickets_remaining
        );
    } else {
        tracing::debug!(
            "Subscription {} not found in database, skipping deletion",
            subscription_id
        );
    }

    Ok(())
}
