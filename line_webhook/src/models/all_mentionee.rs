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

/// AllMentionee : Mentioned target is entire group
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllMentionee {
    /// Mentioned target.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Index position of the user mention for a character in text, with the first character being at position 0.
    #[serde(rename = "index")]
    pub index: i32,
    /// The length of the text of the mentioned user. For a mention @example, 8 is the length.
    #[serde(rename = "length")]
    pub length: i32,
}

impl AllMentionee {
    /// Mentioned target is entire group
    pub fn new(r#type: String, index: i32, length: i32) -> AllMentionee {
        AllMentionee {
            r#type: Some(r#type),
            index,
            length,
        }
    }
}
