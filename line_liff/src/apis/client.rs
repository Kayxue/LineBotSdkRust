use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    liff_api: Box<dyn crate::apis::LiffApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            liff_api: Box::new(crate::apis::LiffApiClient::new(rc.clone())),
        }
    }

    pub fn liff_api(&self) -> &dyn crate::apis::LiffApi{
        self.liff_api.as_ref()
    }

}
