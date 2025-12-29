use anyhow::{anyhow, Result};
use std::collections::HashMap;
use stripe::{
    CancelPaymentIntent, Client, CreatePaymentIntent, CreateRefund, Currency, PaymentIntent,
    PaymentIntentId, Refund,
};

/// Fixed exchange rate: 1 USD = 25,000 VND
const VND_TO_USD_RATE: f64 = 25_000.0;

/// Minimum Stripe charge in USD cents
const MIN_CHARGE_CENTS: i64 = 50;

#[derive(Clone)]
pub struct StripePayments {
    client: Client,
}

impl StripePayments {
    pub fn new(secret_key: String) -> Self {
        let client = Client::new(secret_key);
        Self { client }
    }

    /// Create payment intent for booking
    ///
    /// Converts VND to USD using fixed exchange rate (1 USD = 25,000 VND).
    /// Stripe minimum charge is $0.50 (50 cents).
    pub async fn create_payment_intent(
        &self,
        amount_vnd: i32,
        booking_id: &str,
        user_id: &str,
        booking_code: &str,
    ) -> Result<PaymentIntent> {
        // Convert VND to USD cents
        let amount_usd_cents = ((amount_vnd as f64 / VND_TO_USD_RATE) * 100.0).round() as i64;
        let amount_usd_cents = amount_usd_cents.max(MIN_CHARGE_CENTS);

        // Build metadata for webhook correlation
        let mut metadata = HashMap::new();
        metadata.insert("booking_id".to_string(), booking_id.to_string());
        metadata.insert("user_id".to_string(), user_id.to_string());
        metadata.insert("booking_code".to_string(), booking_code.to_string());
        metadata.insert("amount_vnd".to_string(), amount_vnd.to_string());

        let description = format!("Loafy Club Booking: {}", booking_code);
        let mut create_intent = CreatePaymentIntent::new(amount_usd_cents, Currency::USD);
        create_intent.metadata = Some(metadata);
        create_intent.description = Some(&description);
        create_intent.automatic_payment_methods = Some(
            stripe::CreatePaymentIntentAutomaticPaymentMethods {
                enabled: true,
                allow_redirects: None,
            },
        );

        let payment_intent = PaymentIntent::create(&self.client, create_intent)
            .await
            .map_err(|e| anyhow!("Failed to create PaymentIntent: {}", e))?;

        tracing::info!(
            "Created PaymentIntent {} for booking {} ({}c USD from {} VND)",
            payment_intent.id,
            booking_id,
            amount_usd_cents,
            amount_vnd
        );

        Ok(payment_intent)
    }

    /// Get payment intent by ID
    pub async fn get_payment_intent(&self, intent_id: &PaymentIntentId) -> Result<PaymentIntent> {
        let payment_intent = PaymentIntent::retrieve(&self.client, intent_id, &[])
            .await
            .map_err(|e| anyhow!("Failed to retrieve PaymentIntent: {}", e))?;

        Ok(payment_intent)
    }

    /// Cancel a payment intent (use when booking is cancelled before payment)
    pub async fn cancel_payment_intent(
        &self,
        intent_id: &PaymentIntentId,
    ) -> Result<PaymentIntent> {
        let payment_intent =
            PaymentIntent::cancel(&self.client, intent_id, CancelPaymentIntent::default())
                .await
                .map_err(|e| anyhow!("Failed to cancel PaymentIntent: {}", e))?;

        tracing::info!("Cancelled PaymentIntent {}", intent_id);
        Ok(payment_intent)
    }

    /// Refund a completed payment
    pub async fn refund_payment(&self, payment_intent_id: &PaymentIntentId) -> Result<Refund> {
        let mut params = CreateRefund::default();
        params.payment_intent = Some(payment_intent_id.clone());

        let refund = Refund::create(&self.client, params)
            .await
            .map_err(|e| anyhow!("Failed to create refund: {}", e))?;

        tracing::info!(
            "Created refund {} for PaymentIntent {}",
            refund.id,
            payment_intent_id
        );
        Ok(refund)
    }
}
