use anyhow::{anyhow, Result};
use std::collections::HashMap;
use stripe::{
    CancelSubscription, CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, Customer, CustomerId, ListCustomers,
    Subscription, SubscriptionId, UpdateSubscription,
};

#[derive(Clone)]
pub struct StripeSubscriptions {
    client: Client,
    price_id: String,
}

impl StripeSubscriptions {
    pub fn new(secret_key: String, price_id: String) -> Self {
        let client = Client::new(secret_key);
        Self { client, price_id }
    }

    /// Get or create a Stripe Customer for the user
    /// Uses user_id in metadata to find existing customer
    pub async fn get_or_create_customer(
        &self,
        user_id: &str,
        email: &str,
        name: Option<&str>,
    ) -> Result<Customer> {
        // First, try to find existing customer by metadata
        let mut list_params = ListCustomers::new();
        list_params.email = Some(email);
        list_params.limit = Some(1);

        let customers = Customer::list(&self.client, &list_params)
            .await
            .map_err(|e| anyhow!("Failed to list customers: {}", e))?;

        // Check if any customer has matching user_id in metadata
        for customer in customers.data {
            if let Some(metadata) = &customer.metadata {
                if metadata.get("user_id") == Some(&user_id.to_string()) {
                    tracing::debug!("Found existing Stripe customer {} for user {}", customer.id, user_id);
                    return Ok(customer);
                }
            }
        }

        // Create new customer
        let mut metadata = HashMap::new();
        metadata.insert("user_id".to_string(), user_id.to_string());

        let mut create_customer = CreateCustomer::new();
        create_customer.email = Some(email);
        create_customer.name = name;
        create_customer.metadata = Some(metadata);

        let customer = Customer::create(&self.client, create_customer)
            .await
            .map_err(|e| anyhow!("Failed to create customer: {}", e))?;

        tracing::info!("Created Stripe customer {} for user {}", customer.id, user_id);
        Ok(customer)
    }

    /// Create a Checkout Session for subscription purchase
    /// Returns the checkout URL to redirect the user to
    pub async fn create_checkout_session(
        &self,
        customer_id: &str,
        user_id: &str,
        success_url: &str,
        cancel_url: &str,
    ) -> Result<CheckoutSession> {
        let mut metadata = HashMap::new();
        metadata.insert("user_id".to_string(), user_id.to_string());
        metadata.insert("subscription_type".to_string(), "loafy_club".to_string());

        let customer_id = customer_id
            .parse::<CustomerId>()
            .map_err(|e| anyhow!("Invalid customer ID: {}", e))?;

        let line_items = vec![CreateCheckoutSessionLineItems {
            price: Some(self.price_id.clone()),
            quantity: Some(1),
            ..Default::default()
        }];

        let mut create_session = CreateCheckoutSession::new();
        create_session.customer = Some(customer_id);
        create_session.mode = Some(CheckoutSessionMode::Subscription);
        create_session.line_items = Some(line_items);
        create_session.success_url = Some(success_url);
        create_session.cancel_url = Some(cancel_url);
        create_session.metadata = Some(metadata);
        create_session.subscription_data = Some(stripe::CreateCheckoutSessionSubscriptionData {
            metadata: Some({
                let mut meta = HashMap::new();
                meta.insert("user_id".to_string(), user_id.to_string());
                meta
            }),
            ..Default::default()
        });

        let session = CheckoutSession::create(&self.client, create_session)
            .await
            .map_err(|e| anyhow!("Failed to create checkout session: {}", e))?;

        tracing::info!(
            "Created checkout session {} for user {}",
            session.id,
            user_id
        );

        Ok(session)
    }

    /// Cancel subscription at period end (disable auto-renew)
    /// User keeps access until the current period ends
    pub async fn cancel_at_period_end(&self, subscription_id: &str) -> Result<Subscription> {
        let subscription_id = subscription_id
            .parse::<SubscriptionId>()
            .map_err(|e| anyhow!("Invalid subscription ID: {}", e))?;

        let mut update = UpdateSubscription::new();
        update.cancel_at_period_end = Some(true);

        let subscription = Subscription::update(&self.client, &subscription_id, update)
            .await
            .map_err(|e| anyhow!("Failed to cancel subscription: {}", e))?;

        tracing::info!(
            "Set subscription {} to cancel at period end",
            subscription_id
        );

        Ok(subscription)
    }

    /// Resume subscription (re-enable auto-renew before period ends)
    pub async fn resume_subscription(&self, subscription_id: &str) -> Result<Subscription> {
        let subscription_id = subscription_id
            .parse::<SubscriptionId>()
            .map_err(|e| anyhow!("Invalid subscription ID: {}", e))?;

        let mut update = UpdateSubscription::new();
        update.cancel_at_period_end = Some(false);

        let subscription = Subscription::update(&self.client, &subscription_id, update)
            .await
            .map_err(|e| anyhow!("Failed to resume subscription: {}", e))?;

        tracing::info!("Resumed subscription {}", subscription_id);

        Ok(subscription)
    }

    /// Get subscription details from Stripe
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<Subscription> {
        let subscription_id = subscription_id
            .parse::<SubscriptionId>()
            .map_err(|e| anyhow!("Invalid subscription ID: {}", e))?;

        let subscription = Subscription::retrieve(&self.client, &subscription_id, &[])
            .await
            .map_err(|e| anyhow!("Failed to retrieve subscription: {}", e))?;

        Ok(subscription)
    }

    /// Immediately cancel a subscription (for admin use or special cases)
    #[allow(dead_code)]
    pub async fn cancel_immediately(&self, subscription_id: &str) -> Result<Subscription> {
        let subscription_id = subscription_id
            .parse::<SubscriptionId>()
            .map_err(|e| anyhow!("Invalid subscription ID: {}", e))?;

        let subscription =
            Subscription::cancel(&self.client, &subscription_id, CancelSubscription::default())
                .await
                .map_err(|e| anyhow!("Failed to cancel subscription: {}", e))?;

        tracing::info!("Immediately cancelled subscription {}", subscription_id);

        Ok(subscription)
    }
}
