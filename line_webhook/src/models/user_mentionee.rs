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

/// UserMentionee : Mentioned target is user
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserMentionee {
    /// Mentioned target.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Index position of the user mention for a character in text, with the first character being at position 0.
    #[serde(rename = "index")]
    pub index: i32,
    /// The length of the text of the mentioned user. For a mention @example, 8 is the length.
    #[serde(rename = "length")]
    pub length: i32,
    /// User ID of the mentioned user. Only included if mention.mentions[].type is user and the user consents to the LINE Official Account obtaining their user profile information.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Whether the mentioned user is the bot that receives the webhook.
    #[serde(rename = "isSelf", skip_serializing_if = "Option::is_none")]
    pub is_self: Option<bool>,
}

impl UserMentionee {
    /// Mentioned target is user
    pub fn new(r#type: String, index: i32, length: i32) -> UserMentionee {
        UserMentionee {
            r#type: Some(r#type),
            index,
            length,
            user_id: None,
            is_self: None,
        }
    }
}
