use crate::{
    config::Config, db_conn::DbConn, handlers::shopify_handler, models::shopify_integration,
    with_config, with_db_conn, with_reqwest_client, ConfirmQueryParams, InstallQueryParams,
};
use reqwest::Client;
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix_install() -> BoxedFilter<()> {
    warp::path("shopify_install").boxed()
}

fn confirmation_path() -> BoxedFilter<()> {
    warp::path("shopify_confirm").boxed()
}

pub fn shopify_install(
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
) -> BoxedFilter<(InstallQueryParams, Arc<Config>, Arc<DbConn>)> {
    warp::get()
        .and(path_prefix_install())
        .and(warp::query::query::<InstallQueryParams>())
        .and(with_config(config))
        .and(with_db_conn(db_conn))
        .boxed()
}

pub fn shopify_confirm(
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    client: Arc<Client>,
) -> BoxedFilter<(
    ConfirmQueryParams,
    Arc<Config>,
    Arc<DbConn>,
    shopify_integration::ShopifyIntegration,
    Arc<Client>,
)> {
    warp::get()
        .and(confirmation_path())
        .and(warp::query::query::<ConfirmQueryParams>())
        .and_then(shopify_handler::validate_domain_parameter)
        .and(with_config(config))
        .and_then(shopify_handler::validate_hmac)
        .untuple_one()
        .and(with_db_conn(db_conn))
        .and_then(shopify_handler::find_install_request)
        .untuple_one()
        .and(with_reqwest_client(client))
        .boxed()
}
