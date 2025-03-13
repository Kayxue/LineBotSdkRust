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

/// ModuleBot : basic information about the bot.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleBot {
    /// Bot's user ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Bot's basic ID
    #[serde(rename = "basicId")]
    pub basic_id: String,
    /// Bot's premium ID. Not included in the response if the premium ID isn't set.
    #[serde(rename = "premiumId", skip_serializing_if = "Option::is_none")]
    pub premium_id: Option<String>,
    /// Bot's display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Profile image URL. Image URL starting with `https://`. Not included in the response if the bot doesn't have a profile image.
    #[serde(rename = "pictureUrl", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

impl ModuleBot {
    /// basic information about the bot.
    pub fn new(user_id: String, basic_id: String, display_name: String) -> ModuleBot {
        ModuleBot {
            user_id,
            basic_id,
            premium_id: None,
            display_name,
            picture_url: None,
        }
    }
}

