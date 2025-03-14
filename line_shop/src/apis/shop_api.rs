/*
 * Mission Stickers API
 *
 * This document describes LINE Mission Stickers API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use futures::Future;

use crate::models;
use super::{Error, configuration};
use super::request as __internal_request;

pub struct ShopApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> ShopApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ShopApiClient<C> {
        ShopApiClient {
            configuration,
        }
    }
}

pub trait ShopApi: Send + Sync {
    fn mission_sticker_v3(&self, mission_sticker_request: models::MissionStickerRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
}

impl<C: Connect>ShopApi for ShopApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn mission_sticker_v3(&self, mission_sticker_request: models::MissionStickerRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/shop/v3/mission".to_string())
        ;
        req = req.with_body_param(mission_sticker_request);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
