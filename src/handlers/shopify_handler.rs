use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, services::shopify_service,
    AccessTokenResponse, ConfirmQueryParams, InstallQueryParams,
};
use reqwest::Client;
use std::sync::Arc;
use warp::{reject, Filter, Rejection};

#[derive(Debug)]
pub struct InvalidDomainError;

impl reject::Reject for InvalidDomainError {}

// params: ConfirmQueryParams,
// config: Arc<Config>,
// db_conn: Arc<DbConn>,
// client: Arc<Client>,

pub async fn validate_domain(params: ConfirmQueryParams) -> Result<(ConfirmQueryParams), Rejection> {
    if shopify_service::is_valid_shop_domain(&params) {
        Ok(params)
    } else {
        Err(reject::custom(InvalidDomainError))
    }
}
//
// pub async fn validate_hmac(params: ConfirmQueryParams,config: Arc<Config>) -> Result<(ConfirmQueryParams,Arc<Config>), Rejection> {
//     if shopify_service::is_valid_shop_domain(&params) {
//         Ok(params)
//     } else {
//         Err(reject::custom(InvalidDomainError))
//     }
// }
