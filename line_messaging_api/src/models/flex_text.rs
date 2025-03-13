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
pub struct FlexText {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<i32>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    #[serde(rename = "gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<Gravity>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Weight>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(rename = "decoration", skip_serializing_if = "Option::is_none")]
    pub decoration: Option<Decoration>,
    #[serde(rename = "wrap", skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(rename = "lineSpacing", skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<String>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "offsetTop", skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(rename = "offsetBottom", skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(rename = "offsetStart", skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(rename = "offsetEnd", skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<models::Action>>,
    #[serde(rename = "maxLines", skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<i32>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<models::FlexSpan>>,
    #[serde(rename = "adjustMode", skip_serializing_if = "Option::is_none")]
    pub adjust_mode: Option<AdjustMode>,
    #[serde(rename = "scaling", skip_serializing_if = "Option::is_none")]
    pub scaling: Option<bool>,
}

impl FlexText {
    pub fn new(r#type: String) -> FlexText {
        FlexText {
            r#type: Some(r#type),
            flex: None,
            text: None,
            size: None,
            align: None,
            gravity: None,
            color: None,
            weight: None,
            style: None,
            decoration: None,
            wrap: None,
            line_spacing: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            action: None,
            max_lines: None,
            contents: None,
            adjust_mode: None,
            scaling: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Align {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
}

impl Default for Align {
    fn default() -> Align {
        Self::Start
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gravity {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "center")]
    Center,
}

impl Default for Gravity {
    fn default() -> Gravity {
        Self::Top
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Weight {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "bold")]
    Bold,
}

impl Default for Weight {
    fn default() -> Weight {
        Self::Regular
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "italic")]
    Italic,
}

impl Default for Style {
    fn default() -> Style {
        Self::Normal
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Decoration {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "line-through")]
    LineThrough,
}

impl Default for Decoration {
    fn default() -> Decoration {
        Self::None
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

impl Default for Position {
    fn default() -> Position {
        Self::Relative
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdjustMode {
    #[serde(rename = "shrink-to-fit")]
    ShrinkToFit,
}

impl Default for AdjustMode {
    fn default() -> AdjustMode {
        Self::ShrinkToFit
    }
}
