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
pub struct LocationMessageContent {
    /// Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Message ID
    #[serde(rename = "id")]
    pub id: String,
    /// Title
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Latitude
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Longitude
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

impl LocationMessageContent {
    pub fn new(
        r#type: String,
        id: String,
        latitude: f64,
        longitude: f64,
    ) -> LocationMessageContent {
        LocationMessageContent {
            r#type: Some(r#type),
            id,
            title: None,
            address: None,
            latitude,
            longitude,
        }
    }
}
