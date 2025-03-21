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
pub struct RichMenuRequest {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::RichMenuSize>>,
    /// `true` to display the rich menu by default. Otherwise, `false`.
    #[serde(rename = "selected", skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// Name of the rich menu. This value can be used to help manage your rich menus and is not displayed to users.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Text displayed in the chat bar
    #[serde(rename = "chatBarText", skip_serializing_if = "Option::is_none")]
    pub chat_bar_text: Option<String>,
    /// Array of area objects which define the coordinates and size of tappable areas
    #[serde(rename = "areas", skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<models::RichMenuArea>>,
}

impl RichMenuRequest {
    pub fn new() -> RichMenuRequest {
        RichMenuRequest {
            size: None,
            selected: None,
            name: None,
            chat_bar_text: None,
            areas: None,
        }
    }
}

