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
pub struct UnsendDetail {
    /// The message ID of the unsent message
    #[serde(rename = "messageId")]
    pub message_id: String,
}

impl UnsendDetail {
    pub fn new(message_id: String) -> UnsendDetail {
        UnsendDetail {
            message_id,
        }
    }
}

