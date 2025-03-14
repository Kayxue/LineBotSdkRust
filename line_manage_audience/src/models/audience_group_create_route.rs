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

/// AudienceGroupCreateRoute : How the audience was created. One of:  - `OA_MANAGER`: Audience created with [LINE Official Account Manager](https://manager.line.biz/). - `MESSAGING_API`: Audience created with Messaging API. - `POINT_AD`: Audience created with [LINE Points Ads](https://www.linebiz.com/jp/service/line-point-ad/) (Japanese only). - `AD_MANAGER`: Audience created with [LINE Ads](https://admanager.line.biz/). 
/// How the audience was created. One of:  - `OA_MANAGER`: Audience created with [LINE Official Account Manager](https://manager.line.biz/). - `MESSAGING_API`: Audience created with Messaging API. - `POINT_AD`: Audience created with [LINE Points Ads](https://www.linebiz.com/jp/service/line-point-ad/) (Japanese only). - `AD_MANAGER`: Audience created with [LINE Ads](https://admanager.line.biz/). 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudienceGroupCreateRoute {
    #[serde(rename = "OA_MANAGER")]
    OaManager,
    #[serde(rename = "MESSAGING_API")]
    MessagingApi,
    #[serde(rename = "POINT_AD")]
    PointAd,
    #[serde(rename = "AD_MANAGER")]
    AdManager,

}

impl std::fmt::Display for AudienceGroupCreateRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OaManager => write!(f, "OA_MANAGER"),
            Self::MessagingApi => write!(f, "MESSAGING_API"),
            Self::PointAd => write!(f, "POINT_AD"),
            Self::AdManager => write!(f, "AD_MANAGER"),
        }
    }
}

impl Default for AudienceGroupCreateRoute {
    fn default() -> AudienceGroupCreateRoute {
        Self::OaManager
    }
}

