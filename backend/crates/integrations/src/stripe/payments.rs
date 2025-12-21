use anyhow::Result;
use serde::{Deserialize, Serialize};

// Temporary stub types until we integrate proper Stripe library
#[derive(Clone)]
#[allow(dead_code)]
pub struct Client {
    secret_key: String,
}

impl Client {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub client_secret: Option<String>,
}

pub type PaymentIntentId = String;

#[derive(Clone)]
#[allow(dead_code)]
pub struct StripePayments {
    client: Client,
}

impl StripePayments {
    pub fn new(secret_key: String) -> Self {
        let client = Client::new(secret_key);
        Self { client }
    }

    /// Create payment intent for booking
    pub async fn create_payment_intent(
        &self,
        _amount_vnd: i32,
        booking_id: &str,
    ) -> Result<PaymentIntent> {
        // TODO: Implement proper Stripe API call
        // For now, return a stub response
        tracing::warn!("Using stub Stripe integration - payment intent not actually created");
        Ok(PaymentIntent {
            id: format!("pi_stub_{}", booking_id),
            client_secret: Some(format!("pi_stub_{}_secret", booking_id)),
        })
    }

    /// Get payment intent status
    pub async fn get_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent> {
        // TODO: Implement proper Stripe API call
        tracing::warn!("Using stub Stripe integration");
        Ok(PaymentIntent {
            id: intent_id.to_string(),
            client_secret: Some(format!("{}_secret", intent_id)),
        })
    }

    /// Refund a payment
    pub async fn refund_payment(&self, _payment_intent_id: &str) -> Result<()> {
        // TODO: Implement proper Stripe refund
        tracing::warn!("Using stub Stripe integration - refund not actually processed");
        Ok(())
    }
}
