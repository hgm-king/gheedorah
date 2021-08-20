use crate::{config::Config, ConfirmQueryParams};
use graphql_client::{GraphQLQuery, Response};
use log::info;
use reqwest::{header, Client};
use std::error::Error;
use std::sync::Arc;
use warp::http::Method;

type URL = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/shopify_schema.graphql",
    query_path = "graphql/create_product.graphql",
    response_derives = "Debug"
)]
struct CreateGiftCardProduct;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/shopify_schema.graphql",
    query_path = "graphql/create_products_update_webhook.graphql",
    response_derives = "Debug"
)]
struct CreateProductsWebhook;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/shopify_schema.graphql",
    query_path = "graphql/create_orders_create_webhook.graphql",
    response_derives = "Debug"
)]
struct CreateOrdersWebhook;

fn generate_headers(access_token: &str) -> Result<header::HeaderMap, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(access_token)?;
    auth_value.set_sensitive(true);
    headers.insert("X-Shopify-Access-Token", auth_value);
    Ok(headers)
}

pub async fn create_product(
    params: &ConfirmQueryParams,
    config: Arc<Config>,
    client: Arc<Client>,
    access_token: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let variables = create_gift_card_product::Variables {};

    let uri = config.shopify.get_graphql_url(params.shop.clone());
    let headers = generate_headers(&access_token)?;
    let builder = client.request(Method::POST, uri).headers(headers);

    let response_body = query_graphql::<CreateGiftCardProduct>(builder, variables).await?;
    let response_data: create_gift_card_product::ResponseData = response_body.data.unwrap();

    let id = response_data.product_create.unwrap().product.unwrap().id;
    info!("Created CreateGiftCardProduct with id {}", id);
    Ok(id)
}

pub async fn create_products_webhook(
    params: &ConfirmQueryParams,
    config: Arc<Config>,
    client: Arc<Client>,
    access_token: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let variables = create_products_webhook::Variables {
        callback_url: config.shopify.get_product_webhook_uri()
    };

    let uri = config.shopify.get_graphql_url(params.shop.clone());
    let headers = generate_headers(&access_token)?;

    let builder = client.request(Method::POST, uri).headers(headers);

    let response_body = query_graphql::<CreateProductsWebhook>(builder, variables).await?;
    let response_data: create_products_webhook::ResponseData = response_body.data.unwrap();

    let id = response_data
        .webhook_subscription_create
        .unwrap()
        .webhook_subscription
        .unwrap()
        .id;
    info!("Created CreateProductsWebhook with id {}", id);
    Ok(id)
}

pub async fn create_orders_webhook(
    params: &ConfirmQueryParams,
    config: Arc<Config>,
    client: Arc<Client>,
    access_token: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let variables = create_orders_webhook::Variables {
        callback_url: config.shopify.get_order_webhook_uri()
    };

    let uri = config.shopify.get_graphql_url(params.shop.clone());
    let headers = generate_headers(&access_token)?;

    let builder = client.request(Method::POST, uri).headers(headers);

    let response_body = query_graphql::<CreateOrdersWebhook>(builder, variables).await?;
    let response_data: create_orders_webhook::ResponseData = response_body.data.unwrap();

    let id = response_data
        .webhook_subscription_create
        .unwrap()
        .webhook_subscription
        .unwrap()
        .id;
    info!("Created CreateOrdersWebhook with id {}", id);
    Ok(id)
}

async fn query_graphql<Q: GraphQLQuery>(
    builder: reqwest::RequestBuilder,
    variables: Q::Variables,
) -> Result<Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = builder.json(&body).send().await?;
    Ok(reqwest_response.json().await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    fn mock_params() -> ConfirmQueryParams {
        ConfirmQueryParams {
            code: String::from("314159"),
            hmac: String::from("00d39b4e40556ad1f8c8a5c673975e62abc8e0f2574d99a1934e2e881350a710"),
            host: String::from("26535"),
            timestamp: String::from("1337178173"),
            state: String::from("89793"),
            shop: String::from("shop.myshopify.com"),
        }
    }

    #[tokio::test]
    async fn it_can_run_create_product_graphql() {
        let shop = String::from("leboulangerie.myshopify.com");
        let access_token = String::from("shush");

        let mut params = mock_params();
        params.shop = shop.clone();

        let mut config = config::generate_mocking_config();
        config.shopify.set_graphql_domain(mockito::server_url());

        let client = reqwest::Client::new();
        let _m = mockito::mock("POST", mockito::Matcher::Exact(config.shopify.graphql_path.clone()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                "{\"data\":{\"productCreate\":{\"product\":{\"id\":\"gid:\\/\\/shopify\\/Product\\/6887105790141\"}}},\"extensions\":{\"cost\":{\"requestedQueryCost\":10,\"actualQueryCost\":10,\"throttleStatus\":{\"maximumAvailable\":1000.0,\"currentlyAvailable\":990,\"restoreRate\":50.0}}}}"
            )
            .create();

        let res = create_product(&params, Arc::new(config), Arc::new(client), access_token)
            .await
            .unwrap();

        assert_eq!(res, "gid://shopify/Product/6887105790141");
    }

    #[tokio::test]
    async fn it_can_run_create_webhook_product_graphql() {
        let shop = String::from("labanque.myshopify.com");
        let access_token = String::from("shush");

        let mut params = mock_params();
        params.shop = shop.clone();

        let mut config = config::generate_mocking_config();
        config.shopify.set_graphql_domain(mockito::server_url());

        let client = reqwest::Client::new();
        let _m = mockito::mock("POST", mockito::Matcher::Exact(config.shopify.graphql_path.clone()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                "{\"data\":{\"webhookSubscriptionCreate\":{\"userErrors\":[],\"webhookSubscription\":{\"id\":\"gid:\\/\\/shopify\\/WebhookSubscription\\/1055293472957\"}}},\"extensions\":{\"cost\":{\"requestedQueryCost\":10,\"actualQueryCost\":10,\"throttleStatus\":{\"maximumAvailable\":1000,\"currentlyAvailable\":990,\"restoreRate\":50}}}}"
            )
            .create();

        let res =
            create_products_webhook(&params, Arc::new(config), Arc::new(client), access_token)
                .await
                .unwrap();

        assert_eq!(res, "gid://shopify/WebhookSubscription/1055293472957");
    }

    #[tokio::test]
    async fn it_can_run_create_webhook_order_graphql() {
        let shop = String::from("labanque.myshopify.com");
        let access_token = String::from("shush");

        let mut params = mock_params();
        params.shop = shop.clone();

        let mut config = config::generate_mocking_config();
        config.shopify.set_graphql_domain(mockito::server_url());

        let client = reqwest::Client::new();
        let _m = mockito::mock("POST", mockito::Matcher::Exact(config.shopify.graphql_path.clone()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                "{\"data\":{\"webhookSubscriptionCreate\":{\"userErrors\":[],\"webhookSubscription\":{\"id\":\"gid:\\/\\/shopify\\/WebhookSubscription\\/1055293472957\"}}},\"extensions\":{\"cost\":{\"requestedQueryCost\":10,\"actualQueryCost\":10,\"throttleStatus\":{\"maximumAvailable\":1000,\"currentlyAvailable\":990,\"restoreRate\":50}}}}"
            )
            .create();

        let res = create_orders_webhook(&params, Arc::new(config), Arc::new(client), access_token)
            .await
            .unwrap();

        assert_eq!(res, "gid://shopify/WebhookSubscription/1055293472957");
    }
}
