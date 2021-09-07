// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{UsageRecordId, SubscriptionItemId};
use crate::params::{Object, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "UsageRecord".
///
/// For more details see <https://stripe.com/docs/api/usage_records/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: UsageRecordId,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The usage quantity for the specified date.
    pub quantity: u64,

    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,

    /// The timestamp when this usage occurred.
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateUsageRecord {
    pub quantity: u64,
    pub timestamp: Timestamp,
    /// Can be either set or increment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl UsageRecord {
    /// Creates a usage record.
    pub fn create(
        client: &Client,
        subscription_id: &SubscriptionItemId,
        params: CreateUsageRecord,
    ) -> Response<UsageRecord> {
        client.post_form(&format!("/subscription_items/{}/usage_records", subscription_id), params)
    }
}

impl Object for UsageRecord {
    type Id = UsageRecordId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "usage_record"
    }
}
