use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, utils::gen_uuid,
    AccessTokenResponse, ConfirmQueryParams, InstallQueryParams,
};
use hmac::{Hmac, Mac, NewMac};
use lazy_regex::regex;
use reqwest::Client;
use sha2::Sha256;
use std::error::Error;
use std::sync::Arc;
use warp::http::Uri;

pub async fn get_access_token(
    client: Arc<Client>,
    form_body: Vec<(String, String)>,
    shopify_url: String,
) -> Result<AccessTokenResponse, reqwest::Error> {
    let access_token_json: AccessTokenResponse = client
        .post(format!("{}/admin/oauth/access_token", shopify_url))
        .form(&form_body)
        .send()
        .await?
        .json()
        .await?;

    Ok(access_token_json)
}

// need to confirm that the domain coming from params is from shopify
pub fn is_valid_shop_domain(params: &ConfirmQueryParams) -> bool {
    let r = regex!("^[a-zA-Z0-9][a-zA-Z0-9\\-]*\\.myshopify\\.com$");
    r.is_match(&params.shop)
}

pub fn validate_shopify_hmac(
    params: ConfirmQueryParams,
    config: Arc<Config>,
) -> Result<(), crypto_mac::MacError> {
    let secret_bytes = &config.shopify_api_secret.as_bytes();
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_bytes).map_err(|e| crypto_mac::MacError)?;

    let params_code = convert_query_params_to_hmac_code(&params);
    mac.update(&params_code.as_bytes());

    let hmac_bytes = hex::decode(params.hmac).map_err(|e| crypto_mac::MacError)?;
    mac.verify(&hmac_bytes)?;

    Ok(())
}

pub fn find_shopify_integration_request_from_params(
    params: ConfirmQueryParams,
    db_conn: Arc<DbConn>,
) -> Option<shopify_integration::ShopifyIntegration> {
    let conn = &db_conn.get_conn();
    let shop = params.shop.clone();
    let state = params.state;

    match shopify_integration::read_by_shop_and_nonce(conn, shop, state) {
        Err(_) => None,
        Ok(mut shoption) => {
            if 0 < shoption.len() {
                Some(shoption.remove(0))
            } else {
                None
            }
        }
    }
}

// let uri = if config.is_mocking {
//     config.shopify_api_uri.clone()
// } else {
//     format!("{}{}", config.shopify_api_uri.clone(), params.shop)
// };
//
// let access_token_json = shopify_service::get_access_token(client.clone(), form_body, uri)
//     .await
//     .expect("Could not fetch access token!");
//
// // update the shop here
// shopify_connection::update_access_token(&conn, &shop_conn, access_token_json.access_token)
//     .expect("Could not insert to db");

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

// setup the form body to request the access token from shopify api
fn form_body_from_args(api_key: String, api_secret: String, code: String) -> Vec<(String, String)> {
    vec![
        (String::from("client_id"), api_key),
        (String::from("client_secret"), api_secret),
        (String::from("code"), code),
    ]
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::generate_mocking_config;

    #[test]
    fn it_validates_valid_shop_param() {
        let params = ConfirmQueryParams {
            code: String::from(""),
            hmac: String::from(""),
            host: String::from(""),
            timestamp: String::from(""),
            state: String::from(""),
            shop: String::from("big-shop.myshopify.com"),
        };
        assert!(is_valid_shop_domain(&params));
    }

    #[test]
    fn it_doesnt_validates_invalid_shop_param() {
        let params = ConfirmQueryParams {
            code: String::from(""),
            hmac: String::from(""),
            host: String::from(""),
            timestamp: String::from(""),
            state: String::from(""),
            shop: String::from("bass-ackwards.com"),
        };
        assert!(!is_valid_shop_domain(&params));
    }

    #[test]
    fn it_validates_valid_hmac() {
        let mut config = generate_mocking_config();
        config.set_shopify_secret_key(String::from("hush"));

        let params = ConfirmQueryParams {
            code: String::from("0907a61c0c8d55e99db179b68161bc00"),
            hmac: String::from("700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf"),
            host: String::from("YmRyb2NrZXRzdG9yZS5teXNob3BpZnkuY29tL2FkbWlu"),
            timestamp: String::from("1337178173"),
            state: String::from("0.6784241404160823"),
            shop: String::from("some-shop.myshopify.com"),
        };

        assert!(validate_shopify_hmac(params, Arc::new(config)).is_ok());
    }

    #[test]
    fn it_doesnt_validates_invalid_hmac() {
        let mut config = generate_mocking_config();
        config.set_shopify_secret_key(String::from("loud"));

        let params = ConfirmQueryParams {
            code: String::from("0907a61c0c8d55e99db179b68161bc00"),
            hmac: String::from("700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf"),
            host: String::from("YmRyb2NrZXRzdG9yZS5teXNob3BpZnkuY29tL2FkbWlu"),
            timestamp: String::from("1337178173"),
            state: String::from("0.6784241404160823"),
            shop: String::from("some-shop.myshopify.com"),
        };

        assert!(validate_shopify_hmac(params, Arc::new(config)).is_err());
    }
}
