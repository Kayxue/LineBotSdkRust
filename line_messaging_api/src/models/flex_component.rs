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

use super::{
    FlexBox, FlexButton, FlexFiller, FlexIcon, FlexImage, FlexSeparator, FlexSpan, FlexText,
    FlexVideo,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FlexComponent {
    #[serde(rename = "box")]
    FlexBox(FlexBox),
    #[serde(rename = "button")]
    FlexButton(FlexButton),
    #[serde(rename = "filler")]
    FlexFiller(FlexFiller),
    #[serde(rename = "icon")]
    FlexIcon(FlexIcon),
    #[serde(rename = "image")]
    FlexImage(FlexImage),
    #[serde(rename = "separator")]
    FlexSeparator(FlexSeparator),
    #[serde(rename = "span")]
    FlexSpan(FlexSpan),
    #[serde(rename = "text")]
    FlexText(FlexText),
    #[serde(rename = "video")]
    FlexVideo(FlexVideo),
}

impl Default for FlexComponent {
    fn default() -> Self {
        Self::FlexBox(FlexBox::default())
    }
}
