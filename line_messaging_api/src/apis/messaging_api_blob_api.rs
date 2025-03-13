/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
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

pub struct MessagingApiBlobApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> MessagingApiBlobApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> MessagingApiBlobApiClient<C> {
        MessagingApiBlobApiClient {
            configuration,
        }
    }
}

pub trait MessagingApiBlobApi: Send + Sync {
    fn get_message_content(&self, message_id: &str) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn get_message_content_preview(&self, message_id: &str) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn get_message_content_transcoding_by_message_id(&self, message_id: &str) -> Pin<Box<dyn Future<Output = Result<models::GetMessageContentTranscodingResponse, Error>> + Send>>;
    fn get_rich_menu_image(&self, rich_menu_id: &str) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn set_rich_menu_image(&self, rich_menu_id: &str, body: Option<std::path::PathBuf>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
}

impl<C: Connect>MessagingApiBlobApi for MessagingApiBlobApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn get_message_content(&self, message_id: &str) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/v2/bot/message/{messageId}/content".to_string())
        ;
        req = req.with_path_param("messageId".to_string(), message_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_message_content_preview(&self, message_id: &str) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/v2/bot/message/{messageId}/content/preview".to_string())
        ;
        req = req.with_path_param("messageId".to_string(), message_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_message_content_transcoding_by_message_id(&self, message_id: &str) -> Pin<Box<dyn Future<Output = Result<models::GetMessageContentTranscodingResponse, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/v2/bot/message/{messageId}/content/transcoding".to_string())
        ;
        req = req.with_path_param("messageId".to_string(), message_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_rich_menu_image(&self, rich_menu_id: &str) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/v2/bot/richmenu/{richMenuId}/content".to_string())
        ;
        req = req.with_path_param("richMenuId".to_string(), rich_menu_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn set_rich_menu_image(&self, rich_menu_id: &str, body: Option<std::path::PathBuf>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/v2/bot/richmenu/{richMenuId}/content".to_string())
        ;
        req = req.with_path_param("richMenuId".to_string(), rich_menu_id.to_string());
        req = req.with_body_param(body);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
