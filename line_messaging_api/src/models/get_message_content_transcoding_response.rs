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

/// GetMessageContentTranscodingResponse : Transcoding response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMessageContentTranscodingResponse {
    /// The preparation status. One of:  `processing`: Preparing to get content. `succeeded`: Ready to get the content. You can get the content sent by users. `failed`: Failed to prepare to get the content. 
    #[serde(rename = "status")]
    pub status: Status,
}

impl GetMessageContentTranscodingResponse {
    /// Transcoding response
    pub fn new(status: Status) -> GetMessageContentTranscodingResponse {
        GetMessageContentTranscodingResponse {
            status,
        }
    }
}
/// The preparation status. One of:  `processing`: Preparing to get content. `succeeded`: Ready to get the content. You can get the content sent by users. `failed`: Failed to prepare to get the content. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Processing
    }
}

