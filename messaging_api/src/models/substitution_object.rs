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

use super::{EmojiSubstitutionObject, MentionSubstitutionObject};

/// SubstitutionObject : An object that defines the replacement value for a placeholder in the text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SubstitutionObject {
    #[serde(rename = "emoji")]
    EmojiSubstitutionObject(EmojiSubstitutionObject),
    #[serde(rename = "mention")]
    MentionSubstitutionObject(MentionSubstitutionObject),
}

impl Default for SubstitutionObject {
    fn default() -> Self {
        Self::EmojiSubstitutionObject(EmojiSubstitutionObject::default())
    }
}
