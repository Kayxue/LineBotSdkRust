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
pub struct FollowDetail {
    /// Whether a user has added your LINE Official Account as a friend or unblocked.
    #[serde(rename = "isUnblocked")]
    pub is_unblocked: bool,
}

impl FollowDetail {
    pub fn new(is_unblocked: bool) -> FollowDetail {
        FollowDetail {
            is_unblocked,
        }
    }
}

