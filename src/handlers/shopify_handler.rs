use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, services::shopify_service,
    ConfirmQueryParams,
};
use log::error;
use reqwest::Client;
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

#[derive(Debug)]
pub struct AccessTokenError;
impl reject::Reject for AccessTokenError {}

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
        error!("Invalid shop parameter for {:?}", params);
        Err(reject::custom(InvalidDomainError))
    }
}

pub async fn validate_hmac(
    params: ConfirmQueryParams,
    config: Arc<Config>,
) -> Result<(ConfirmQueryParams, Arc<Config>), Rejection> {
    match shopify_service::validate_hmac(&params, config.clone()) {
        Ok(_) => Ok((params, config)),
        Err(_) => {
            error!("Invalid hmac for {:?}", params);
            Err(reject::custom(InvalidHmacError))
        }
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
        Some(shop_integration) => Ok((params, config, db_conn, shop_integration)),
        None => {
            error!("Missing install request for {:?}", params);
            Err(reject::custom(MissingIntegrationError))
        }
    }
}

pub async fn update_with_access_token(
    params: ConfirmQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    shop_integration: shopify_integration::ShopifyIntegration,
    client: Arc<Client>,
) -> Result<
    (
        ConfirmQueryParams,
        Arc<Config>,
        Arc<DbConn>,
        shopify_integration::ShopifyIntegration,
        Arc<Client>,
    ),
    Rejection,
> {
    match shopify_service::update_integration_with_access_token(
        &params,
        config.clone(),
        &db_conn.get_conn(),
        &shop_integration,
        client.clone(),
    )
    .await
    {
        Ok(_) => Ok((params, config, db_conn, shop_integration, client)),
        Err(_) => {
            error!("Could not fetch access token for {:?}", params);
            Err(reject::custom(AccessTokenError))
        }
    }
}

// pub async fn health() -> Result<impl Reply, Rejection> {
//     Ok(reply::json(&HealthResponse::new(String::from("OK"))))
// }
