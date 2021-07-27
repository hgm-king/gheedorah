use env_logger::Env;
use log::info;
use sidecar::{
    config::generate_config,
    db_conn::DbConn,
    handlers::{health_handler, shopify_handler},
    routes::{health_route, shopify_route},
};
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Sidecar 🥃");

    // set up global dependencies, using arc to have shared references across requests
    let config = Arc::new(generate_config());
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let client = Arc::new(reqwest::Client::new());

    // compose our routes and handlers
    let shopify =
        shopify!(config.clone(), db_conn.clone(), client.clone()).with(warp::log("shopify"));
    let end = health!().or(shopify);

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    info!("Listening at {}", &config.app_addr);

    if config.clone().tls {
        info!("TLS Enabled!");

        warp::serve(end)
            .tls()
            .cert_path(config.clone().cert_path.as_ref().unwrap())
            .key_path(config.clone().key_path.as_ref().unwrap())
            .run(socket_address)
            .await;
    } else {
        warp::serve(end).run(socket_address).await;
    }
}
