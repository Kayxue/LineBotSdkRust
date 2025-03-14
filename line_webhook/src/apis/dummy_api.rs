/*
 * Webhook Type Definition
 *
 * Webhook event definition of the LINE Messaging API
 *
 * The version of the OpenAPI document: 1.0.0
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

pub struct DummyApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> DummyApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> DummyApiClient<C> {
        DummyApiClient {
            configuration,
        }
    }
}

pub trait DummyApi: Send + Sync {
    fn callback(&self, callback_request: models::CallbackRequest) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>;
}

impl<C: Connect>DummyApi for DummyApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn callback(&self, callback_request: models::CallbackRequest) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/callback".to_string())
        ;
        req = req.with_body_param(callback_request);

        req.execute(self.configuration.borrow())
    }

}
