use anyhow::{anyhow, Result};
use stripe::{
    Client, CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods,
    CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects, Currency, PaymentIntent,
    PaymentIntentId,
};

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
    pub async fn create_payment_intent(
        &self,
        amount_vnd: i32,
        booking_id: &str,
    ) -> Result<PaymentIntent> {
        // Convert VND to smallest currency unit (no decimals for VND)
        let amount = amount_vnd as i64;

        let mut create_intent = CreatePaymentIntent::new(amount, Currency::VND);
        create_intent.metadata = Some(
            [("booking_id".to_string(), booking_id.to_string())]
                .into_iter()
                .collect(),
        );

        // Enable automatic payment methods
        create_intent.automatic_payment_methods = Some(
            CreatePaymentIntentAutomaticPaymentMethods {
                enabled: true,
                allow_redirects: Some(
                    CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Never,
                ),
            },
        );

        let payment_intent = PaymentIntent::create(&self.client, create_intent).await?;

        Ok(payment_intent)
    }

    /// Get payment intent status
    pub async fn get_payment_intent(&self, intent_id: &str) -> Result<PaymentIntent> {
        let intent_id = PaymentIntentId::from(intent_id);
        let payment_intent = PaymentIntent::retrieve(&self.client, &intent_id, &[]).await?;

        Ok(payment_intent)
    }

    /// Refund a payment
    pub async fn refund_payment(&self, payment_intent_id: &str) -> Result<()> {
        use stripe::{CreateRefund, Refund};

        let mut refund = CreateRefund::new();
        refund.payment_intent = Some(PaymentIntentId::from(payment_intent_id));

        Refund::create(&self.client, refund).await?;

        Ok(())
    }
}
