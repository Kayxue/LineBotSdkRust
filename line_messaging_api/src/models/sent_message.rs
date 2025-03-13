/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentMessage {
    /// ID of the sent message.
    #[serde(rename = "id")]
    pub id: String,
    /// Quote token of the message. Only included when a message object that can be specified as a quote target was sent as a push or reply message. 
    #[serde(rename = "quoteToken", skip_serializing_if = "Option::is_none")]
    pub quote_token: Option<String>,
}

impl SentMessage {
    pub fn new(id: String) -> SentMessage {
        SentMessage {
            id,
            quote_token: None,
        }
    }
}

