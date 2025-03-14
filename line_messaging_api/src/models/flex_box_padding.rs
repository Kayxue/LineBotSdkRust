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

/// FlexBoxPadding : Padding can be specified in pixels, percentage (to the parent box width) or with a keyword. FlexBoxPadding just provides only keywords. 
/// Padding can be specified in pixels, percentage (to the parent box width) or with a keyword. FlexBoxPadding just provides only keywords. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlexBoxPadding {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "xs")]
    Xs,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "md")]
    Md,
    #[serde(rename = "lg")]
    Lg,
    #[serde(rename = "xl")]
    Xl,
    #[serde(rename = "xxl")]
    Xxl,

}

impl std::fmt::Display for FlexBoxPadding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Xs => write!(f, "xs"),
            Self::Sm => write!(f, "sm"),
            Self::Md => write!(f, "md"),
            Self::Lg => write!(f, "lg"),
            Self::Xl => write!(f, "xl"),
            Self::Xxl => write!(f, "xxl"),
        }
    }
}

impl Default for FlexBoxPadding {
    fn default() -> FlexBoxPadding {
        Self::None
    }
}

