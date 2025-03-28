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
pub struct AudioMessageContent {
    /// Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Message ID
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contentProvider")]
    pub content_provider: Box<models::ContentProvider>,
    /// Length of audio file (milliseconds)
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

impl AudioMessageContent {
    pub fn new(
        r#type: String,
        id: String,
        content_provider: models::ContentProvider,
    ) -> AudioMessageContent {
        AudioMessageContent {
            r#type: Some(r#type),
            id,
            content_provider: Box::new(content_provider),
            duration: None,
        }
    }
}
