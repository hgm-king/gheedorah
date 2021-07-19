use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, services::shopify_service,
    ConfirmQueryParams,
};
use std::sync::Arc;
use warp::{reject, Rejection};

#[derive(Debug)]
pub struct InvalidDomainError;
impl reject::Reject for InvalidDomainError {}

#[derive(Debug)]
pub struct InvalidHmacError;
impl reject::Reject for InvalidHmacError {}

#[derive(Debug)]
pub struct MissingIntegrationError;
impl reject::Reject for MissingIntegrationError {}

// params: ConfirmQueryParams,
// config: Arc<Config>,
// db_conn: Arc<DbConn>,
// client: Arc<Client>,

pub async fn validate_domain_parameter(
    params: ConfirmQueryParams,
) -> Result<ConfirmQueryParams, Rejection> {
    if shopify_service::is_valid_shop_domain(&params) {
        Ok(params)
    } else {
        Err(reject::custom(InvalidDomainError))
    }
}

pub async fn validate_hmac(
    params: ConfirmQueryParams,
    config: Arc<Config>,
) -> Result<(ConfirmQueryParams, Arc<Config>), Rejection> {
    match shopify_service::validate_hmac(&params, config.clone()) {
        Ok(_) => Ok((params, config)),
        Err(_) => Err(reject::custom(InvalidHmacError)),
    }
}

pub async fn find_install_request(
    params: ConfirmQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
) -> Result<
    (
        ConfirmQueryParams,
        Arc<Config>,
        Arc<DbConn>,
        shopify_integration::ShopifyIntegration,
    ),
    Rejection,
> {
    match shopify_service::find_integration_request_from_params(&params, &db_conn.get_conn()) {
        Some(shopify_integration) => Ok((params, config, db_conn, shopify_integration)),
        None => Err(reject::custom(MissingIntegrationError)),
    }
}
