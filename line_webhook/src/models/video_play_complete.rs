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
pub struct VideoPlayComplete {
    /// ID used to identify a video. Returns the same value as the trackingId assigned to the video message.
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}

impl VideoPlayComplete {
    pub fn new(tracking_id: String) -> VideoPlayComplete {
        VideoPlayComplete {
            tracking_id,
        }
    }
}

