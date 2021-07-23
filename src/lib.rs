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

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallQueryParams {
    hmac: String,
    shop: String,
    timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfirmQueryParams {
    code: String,
    hmac: String,
    host: String,
    timestamp: String,
    state: String,
    shop: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    access_token: String,
    scope: String,
}

pub fn with_config(config: Arc<Config>) -> warp::filters::BoxedFilter<(Arc<Config>,)> {
    warp::any().map(move || config.clone()).boxed()
}

pub fn with_db_conn(conn: Arc<DbConn>) -> warp::filters::BoxedFilter<(Arc<DbConn>,)> {
    warp::any().map(move || conn.clone()).boxed()
}

pub fn with_reqwest_client(client: Arc<Client>) -> warp::filters::BoxedFilter<(Arc<Client>,)> {
    warp::any().map(move || client.clone()).boxed()
}

#[cfg(feature = "mocks")]
pub fn establish_test_connection() -> PgConnection {
    let database_url = db_test_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn db_test_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set")
}
