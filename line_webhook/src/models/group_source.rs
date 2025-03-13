/*
 * Webhook Type Definition
 *
 * Webhook event definition of the LINE Messaging API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupSource {
    /// source type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Group ID of the source group chat
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// ID of the source user. Only included in message events. Only users of LINE for iOS and LINE for Android are included in userId.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl GroupSource {
    pub fn new(r#type: String, group_id: String) -> GroupSource {
        GroupSource {
            r#type: Some(r#type),
            group_id,
            user_id: None,
        }
    }
}
