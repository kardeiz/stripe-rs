use client::Client;
use error::{Error, ErrorCode};
use params::{Metadata, Timestamp};
use resources::Currency;

/// The resource representing a Stripe refund.
///
/// For more details see https://stripe.com/docs/api#refunds.
#[derive(Debug, Deserialize)]
pub struct Refund {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub charge: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub failure_balance_transaction: Option<String>,
    pub failure_reason: Option<String>,
    pub metadata: Metadata,
    pub reason: Option<String>, // (duplicate, fraudulent, requested_by_customer)
    pub receipt_number: Option<String>,
    pub status: String, // (succeeded, pending, failed, cancelled)
}

#[derive(Default, Serialize)]
pub struct RefundParams<'a> {
    pub charge: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<&'a str>,
}

impl Refund {
    /// Creates a new charge.
    ///
    /// For more details see https://stripe.com/docs/api#create_refund.
    pub fn create(client: &Client, params: RefundParams) -> Result<Refund, Error> {
        client.post("/refunds", params)
    }
}
