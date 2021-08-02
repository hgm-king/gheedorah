use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, services::shopify_service,
    ConfirmQueryParams, ErrorMessage, InstallQueryParams,
};
use log::{error, info};
use reqwest::Client;
use std::convert::Infallible;
use std::error::Error;
use std::sync::Arc;
use warp::{
    http::{StatusCode, Uri},
    reject, Rejection, Reply,
};

#[derive(Debug)]
pub struct CreateIntegrationError;
impl reject::Reject for CreateIntegrationError {}

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

pub async fn create_integration_request(
    params: InstallQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
) -> Result<(InstallQueryParams, Arc<Config>, Arc<DbConn>, String), Rejection> {
    match shopify_service::create_integration_request(&params, db_conn.clone()) {
        Ok(nonce) => Ok((params, config, db_conn, nonce)),
        Err(_) => {
            error!("Could not generate install request for {:?}", params);
            Err(reject::custom(CreateIntegrationError))
        }
    }
}

pub async fn handle_shopify_installation_request(
    params: InstallQueryParams,
    config: Arc<Config>,
    _db_conn: Arc<DbConn>,
    nonce: String,
) -> Result<impl Reply, Rejection> {
    // uri for the conform install page
    let formatted_uri = format!(
        "https://{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state={}",
        params.shop,
        config.shopify_api_key,
        config.shopify_access_scopes,
        config.shopify_installation_confirmation_uri, // probably want to be config
        nonce,
    );
    info!("Redirecting shop {} to uri {}", params.shop, formatted_uri);
    Ok(warp::redirect(
        String::from(formatted_uri).parse::<Uri>().unwrap(),
    ))
}

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
    match shopify_service::find_integration_request_from_params(&params, db_conn.clone()) {
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
        db_conn.clone(),
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

pub async fn handle_shopify_installation_confirmation(
    params: ConfirmQueryParams,
    _config: Arc<Config>,
    _db_conn: Arc<DbConn>,
    _shop_integration: shopify_integration::ShopifyIntegration,
    _client: Arc<Client>,
) -> Result<impl Reply, Rejection> {
    info!("Successfully installed by shop {}; redirecting to uri /", params.shop);
    Ok(warp::redirect(String::from("/").parse::<Uri>().unwrap()))
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn shopify_handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(_cause) => "BAD_REQUEST",
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else if let Some(_) = err.find::<InvalidDomainError>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::BAD_REQUEST;
        message = "INVALID_DOMAIN";
    } else {
        // We should have expected this... Just log and say its a 500
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
