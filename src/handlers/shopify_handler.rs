use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, services::shopify_service,
    utils::gen_uuid, AccessTokenResponse, ConfirmQueryParams, InstallQueryParams,
};
use hmac::{Hmac, Mac, NewMac};
use lazy_regex::regex;
use reqwest::Client;
use sha2::Sha256;
use std::sync::Arc;
use warp::{self, filters::BoxedFilter, http::Uri, reject, Filter};

#[derive(Debug)]
pub struct InvalidDomainError;

impl reject::Reject for InvalidDomainError {}

// when shopkeep requests to install our app,
// they will click a link taking them to this handler.
//
// We redirect them back to their store's domain
// to request access to x,y,z scope/permissions.
//
// e.x. https://{shop}.myshopify.com/admin/oauth/authorize
//          ?client_id={api_key}
//          &scope={scopes}
//          &redirect_uri={redirect_uri}
//          &state={nonce}
//          &grant_options[]={access_mode}
pub async fn shopify_install(
    params: InstallQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let nonce = gen_uuid();
    let conn = &db_conn.get_conn();

    // save install request in db to verify later
    shopify_integration::NewShopifyIntegration::new(params.shop.clone(), nonce.clone())
        .insert(conn);

    // uri for the conform install page
    let formatted_uri = format!(
        "https://{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state={}",
        params.shop,
        config.shopify_api_key,
        "read_orders,write_orders", // probably want to be config
        "https://localhost:3030/shopify_confirm", // probably want to be config
        nonce,
    );

    Ok(warp::redirect(
        String::from(formatted_uri).parse::<Uri>().unwrap(),
    ))
}

// https://example.org/some/redirect/uri?code={authorization_code}&hmac=da9d83c171400a41f8db91a950508985&host={base64_encoded_hostname}&timestamp=1409617544&state={nonce}&shop={shop_origin}
// POST https://{shop}.myshopify.com/admin/oauth/access_token
pub async fn shopify_confirm(
    params: ConfirmQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    client: Arc<Client>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();

    // try and find the shop without the completed request
    let shoption =
        shopify_integration::read_by_shop_and_nonce(&conn, params.shop.clone(), params.state);

    let shop_conn = if let Some(o) = shoption.get(0) {
        o
    } else {
        panic!("Could not find shop and nonce")
    };

    let form_body = form_body_from_args(
        config.shopify_api_key.clone(),
        config.shopify_api_secret.clone(),
        params.code,
    );

    let uri = if config.is_mocking {
        config.shopify_api_uri.clone()
    } else {
        format!("{}{}", config.shopify_api_uri.clone(), params.shop)
    };

    let access_token_json = shopify_service::get_access_token(client.clone(), form_body, uri)
        .await
        .expect("Could not fetch access token!");

    // update the shop here
    shopify_integration::update_access_token(&conn, &shop_conn, access_token_json.access_token)
        .expect("Could not insert to db");

    // gotta figure out the reply later
    Ok(warp::redirect(String::from("/").parse::<Uri>().unwrap()))
}

pub fn is_valid_domain(
    params: ConfirmQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    client: Arc<Client>,
) -> Result<(), warp::Rejection> {
    let r = regex!("^[a-zA-Z0-9][a-zA-Z0-9\\-]*\\.myshopify\\.com$");
    if !r.is_match(&params.shop) {
        Err(reject::custom(InvalidDomainError))
    } else {
        Ok(())
    }
}
//
// let params_code = convert_query_params_to_hmac_code(&params);
// let mut mac = Hmac::<Sha256>::new_from_slice(&config.shopify_api_secret.as_bytes())
//     .expect("HMAC can take key of any size");
// mac.update(&params_code.as_bytes());
// let hmac_bytes = hex::decode(params.hmac).expect("Decoding HMAC failed");
// mac.verify(&hmac_bytes).unwrap();

// setup the form body to request the access token from shopify api
fn form_body_from_args(api_key: String, api_secret: String, code: String) -> Vec<(String, String)> {
    vec![
        (String::from("client_id"), api_key),
        (String::from("client_secret"), api_secret),
        (String::from("code"), code),
    ]
}

// to verify the hmac, we need to turn the query params into the following shape
// "code=0907a61c0c8d55e99db179b68161bc00&shop=some-shop.myshopify.com&state=0.6784241404160823&timestamp=1337178173"
fn convert_query_params_to_hmac_code(params: &ConfirmQueryParams) -> String {
    format!(
        "code={}&shop={}&state={}&timestamp={}",
        params.code.clone(),
        params.shop.clone(),
        params.state.clone(),
        params.timestamp.clone()
    )
}
