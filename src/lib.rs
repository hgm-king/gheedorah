pub mod config;
pub mod db_conn;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;

#[macro_use]
extern crate diesel;

use crate::{config::Config, db_conn::DbConn};
#[cfg(feature = "mocks")]
use diesel::prelude::*;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use warp::Filter;

// The response to the health check
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

impl HealthResponse {
    pub fn new(status: String) -> Self {
        HealthResponse { status }
    }
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// The query params the come over from shopify when a shopkeep requests the install
#[derive(Debug, Deserialize, Serialize)]
pub struct InstallQueryParams {
    hmac: String,
    shop: String,
    timestamp: String,
}

// The query parameters that come over from shopify when a shopkeep confirms the install
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfirmQueryParams {
    code: String,
    hmac: String,
    host: String,
    timestamp: String,
    state: String,
    shop: String,
}

// The response when we request a shop's access token from shopify
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    access_token: String,
    scope: String,
}

// Bring in a config reference to the warp handlers
pub fn with_config(config: Arc<Config>) -> warp::filters::BoxedFilter<(Arc<Config>,)> {
    warp::any().map(move || config.clone()).boxed()
}

// Bring in a db_conn reference to the warp handlers
pub fn with_db_conn(conn: Arc<DbConn>) -> warp::filters::BoxedFilter<(Arc<DbConn>,)> {
    warp::any().map(move || conn.clone()).boxed()
}

// Bring in a rewqest client reference to the warp handlers
pub fn with_reqwest_client(client: Arc<Client>) -> warp::filters::BoxedFilter<(Arc<Client>,)> {
    warp::any().map(move || client.clone()).boxed()
}

// These next two functions make a database connection if we are testing and dont need a config
#[cfg(feature = "mocks")]
pub fn establish_test_connection() -> PgConnection {
    let database_url = db_test_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(feature = "mocks")]
pub fn db_test_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set")
}
