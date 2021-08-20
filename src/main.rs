use env_logger::Env;
use log::info;
use sidecar::{
    config::generate_config,
    db_conn::DbConn,
    handlers::{health_handler, shopify_handler, shopify_order_handler, shopify_product_handler},
    routes::{health_route, shopify_order_route, shopify_product_route, shopify_route},
    services::email_service,
};
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

// we need this to be able to use our route macros
pub mod api;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Booting Sidecar 🥃");

    // set up global dependencies, using arc to have shared references across requests
    let config = Arc::new(generate_config());
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let client = Arc::new(reqwest::Client::new());
    let _mailer = Arc::new(email_service::mock_email_client(config.clone()));

    // configure and compose our routes and handlers
    let shopify =
        shopify!(config.clone(), db_conn.clone(), client.clone()).with(warp::log("shopify"));
    let shopify_order = shopify_order!();
    let shopify_product = shopify_product!();

    // this will log a 404 for each missed route, which is annoying
    let end = health!().or(shopify_order).or(shopify_product).or(shopify);

    // setup our address from the config
    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    info!("Listening at {}", &config.app_addr);

    if config.clone().tls {
        info!("🔐 TLS Enabled!");

        // serve over tls if config says so
        warp::serve(end)
            .tls()
            .cert_path(config.clone().cert_path.as_ref().unwrap())
            .key_path(config.clone().key_path.as_ref().unwrap())
            .run(socket_address)
            .await;
    } else {
        // otherwise serve normally
        warp::serve(end).run(socket_address).await;
    }
}
