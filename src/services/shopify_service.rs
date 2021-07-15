use crate::AccessTokenResponse;
use reqwest::Client;
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
