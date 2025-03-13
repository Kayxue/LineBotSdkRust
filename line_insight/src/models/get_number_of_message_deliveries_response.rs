/*
 * LINE Messaging API(Insight)
 *
 * This document describes LINE Messaging API(Insight).
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GetNumberOfMessageDeliveriesResponse : Get number of message deliveries
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNumberOfMessageDeliveriesResponse {
    /// Status of the counting process.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Number of messages sent to all of this LINE Official Account's friends (broadcast messages).
    #[serde(rename = "broadcast", skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<i64>,
    /// Number of messages sent to some of this LINE Official Account's friends, based on specific attributes (targeted messages).
    #[serde(rename = "targeting", skip_serializing_if = "Option::is_none")]
    pub targeting: Option<i64>,
    /// Number of auto-response messages sent.
    #[serde(rename = "autoResponse", skip_serializing_if = "Option::is_none")]
    pub auto_response: Option<i64>,
    /// Number of greeting messages sent.
    #[serde(rename = "welcomeResponse", skip_serializing_if = "Option::is_none")]
    pub welcome_response: Option<i64>,
    /// Number of messages sent from LINE Official Account Manager [Chat screen](https://www.linebiz.com/jp/manual/OfficialAccountManager/chats/) (only available in Japanese).
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<i64>,
    /// Number of broadcast messages sent with the `Send broadcast message` Messaging API operation.
    #[serde(rename = "apiBroadcast", skip_serializing_if = "Option::is_none")]
    pub api_broadcast: Option<i64>,
    /// Number of push messages sent with the `Send push message` Messaging API operation.
    #[serde(rename = "apiPush", skip_serializing_if = "Option::is_none")]
    pub api_push: Option<i64>,
    /// Number of multicast messages sent with the `Send multicast message` Messaging API operation.
    #[serde(rename = "apiMulticast", skip_serializing_if = "Option::is_none")]
    pub api_multicast: Option<i64>,
    /// Number of narrowcast messages sent with the `Send narrowcast message` Messaging API operation.
    #[serde(rename = "apiNarrowcast", skip_serializing_if = "Option::is_none")]
    pub api_narrowcast: Option<i64>,
    /// Number of replies sent with the `Send reply message` Messaging API operation.
    #[serde(rename = "apiReply", skip_serializing_if = "Option::is_none")]
    pub api_reply: Option<i64>,
}

impl GetNumberOfMessageDeliveriesResponse {
    /// Get number of message deliveries
    pub fn new() -> GetNumberOfMessageDeliveriesResponse {
        GetNumberOfMessageDeliveriesResponse {
            status: None,
            broadcast: None,
            targeting: None,
            auto_response: None,
            welcome_response: None,
            chat: None,
            api_broadcast: None,
            api_push: None,
            api_multicast: None,
            api_narrowcast: None,
            api_reply: None,
        }
    }
}
/// Status of the counting process.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "unready")]
    Unready,
    #[serde(rename = "out_of_service")]
    OutOfService,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ready
    }
}

