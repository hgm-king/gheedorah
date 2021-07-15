use crate::{
    config::Config, db_conn::DbConn, models::shopify_integration, AccessTokenResponse,
    ConfirmQueryParams,
};
use hmac::{Hmac, Mac, NewMac};
use lazy_regex::regex;
use reqwest::Client;
use sha2::Sha256;
use std::error::Error;
use std::sync::Arc;

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

pub fn is_valid_shop_domain(params: ConfirmQueryParams) -> bool {
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
        assert!(is_valid_shop_domain(params));
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
        assert!(!is_valid_shop_domain(params));
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
