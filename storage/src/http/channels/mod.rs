use std::collections::HashMap;

pub mod stop;

/// An notification channel used to watch for resource changes.
#[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WatchableChannel {
    /// A UUID or similar unique string that identifies this channel.
    pub id: String,
    /// An opaque ID that identifies the resource being watched on this channel.
    /// Stable across different API versions.
    pub resource_id: String,
    /// A version-specific identifier for the watched resource.
    pub resource_uri: String,
    /// An arbitrary string delivered to the target address with each notification
    /// delivered over this channel. Optional.
    pub token: String,
    /// Date and time of notification channel expiration. Optional.
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    /// The type of delivery mechanism used for this channel.
    pub r#type: String,
    /// The address where notifications are delivered for this channel.
    pub address: String,
    /// Additional parameters controlling delivery channel behavior. Optional.
    pub params: HashMap<String, String>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    pub payload: bool,
}

#[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// User-specified name for a channel. Needed to unsubscribe.
    pub channel_id: String,
    /// Opaque value generated by GCS representing a bucket. Needed to
    /// unsubscribe.
    pub resource_id: String,
    /// Url used to identify where notifications are sent to.
    pub push_url: String,
    /// Email address of the subscriber.
    pub subscriber_email: String,
    /// Time when the channel was created.
    pub creation_time: Option<chrono::DateTime<chrono::Utc>>,
}
