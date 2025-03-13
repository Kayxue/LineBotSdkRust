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

/// ThingsEvent : Indicates that a user linked a device with LINE.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThingsEvent {
    /// Type of the event
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<models::Source>>,
    /// Time of the event in milliseconds.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "mode")]
    pub mode: models::EventMode,
    /// Webhook Event ID. An ID that uniquely identifies a webhook event. This is a string in ULID format.
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: Box<models::DeliveryContext>,
    /// Reply token used to send reply message to this event
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    #[serde(rename = "things")]
    pub things: Box<models::ThingsContent>,
}

impl ThingsEvent {
    /// Indicates that a user linked a device with LINE.
    pub fn new(
        r#type: String,
        timestamp: i64,
        mode: models::EventMode,
        webhook_event_id: String,
        delivery_context: models::DeliveryContext,
        reply_token: String,
        things: models::ThingsContent,
    ) -> ThingsEvent {
        ThingsEvent {
            r#type: Some(r#type),
            source: None,
            timestamp,
            mode,
            webhook_event_id,
            delivery_context: Box::new(delivery_context),
            reply_token,
            things: Box::new(things),
        }
    }
}
