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
pub struct RoomUserProfileResponse {
    /// User's display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Profile image URL. `https` image URL. Not included in the response if the user doesn't have a profile image.
    #[serde(rename = "pictureUrl", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

impl RoomUserProfileResponse {
    pub fn new(display_name: String, user_id: String) -> RoomUserProfileResponse {
        RoomUserProfileResponse {
            display_name,
            user_id,
            picture_url: None,
        }
    }
}

