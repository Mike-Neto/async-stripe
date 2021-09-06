use serde_derive::Serialize;

use crate::config::{Client, Response};
use crate::ids::SubscriptionItemId;
use crate::resources::{UsageRecord};
use crate::params::Timestamp;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateUsageRecord {
    pub quantity: u64,
    pub timestamp: Timestamp,
    /// Can be either set or increment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

pub fn create(
    client: &Client,
    subscription_id: &SubscriptionItemId,
    params: CreateUsageRecord,
) -> Response<UsageRecord> {
    client.post_form(&format!("/subscription_items/{}/usage_records", subscription_id), params)
}