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

/// FlexSpanSize : Font size in the `size` property of the Flex span component. You can specify the size in pixels or with a keyword. FlexSpanSize just provides only keywords. 
/// Font size in the `size` property of the Flex span component. You can specify the size in pixels or with a keyword. FlexSpanSize just provides only keywords. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlexSpanSize {
    #[serde(rename = "xxs")]
    Xxs,
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
    #[serde(rename = "3xl")]
    Variant3xl,
    #[serde(rename = "4xl")]
    Variant4xl,
    #[serde(rename = "5xl")]
    Variant5xl,

}

impl std::fmt::Display for FlexSpanSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Xxs => write!(f, "xxs"),
            Self::Xs => write!(f, "xs"),
            Self::Sm => write!(f, "sm"),
            Self::Md => write!(f, "md"),
            Self::Lg => write!(f, "lg"),
            Self::Xl => write!(f, "xl"),
            Self::Xxl => write!(f, "xxl"),
            Self::Variant3xl => write!(f, "3xl"),
            Self::Variant4xl => write!(f, "4xl"),
            Self::Variant5xl => write!(f, "5xl"),
        }
    }
}

impl Default for FlexSpanSize {
    fn default() -> FlexSpanSize {
        Self::Xxs
    }
}

