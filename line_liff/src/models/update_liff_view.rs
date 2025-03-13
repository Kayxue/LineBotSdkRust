/*
 * LIFF server API
 *
 * LIFF Server API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLiffView {
    /// Size of the LIFF app view. Specify one of these values: - compact - tall - full 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Endpoint URL. This is the URL of the web app that implements the LIFF app (e.g. https://example.com). Used when the LIFF app is launched using the LIFF URL. The URL scheme must be https. URL fragments (#URL-fragment) can't be specified. 
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// `true` to use the LIFF app in modular mode. When in modular mode, the action button in the header is not displayed. 
    #[serde(rename = "moduleMode", skip_serializing_if = "Option::is_none")]
    pub module_mode: Option<bool>,
}

impl UpdateLiffView {
    pub fn new() -> UpdateLiffView {
        UpdateLiffView {
            r#type: None,
            url: None,
            module_mode: None,
        }
    }
}
/// Size of the LIFF app view. Specify one of these values: - compact - tall - full 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "tall")]
    Tall,
    #[serde(rename = "full")]
    Full,
}

impl Default for Type {
    fn default() -> Type {
        Self::Compact
    }
}

