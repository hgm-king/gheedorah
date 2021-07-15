use crate::{
    config::Config, db_conn::DbConn, with_config, with_db_conn, with_reqwest_client,
    ConfirmQueryParams, InstallQueryParams,
};
use reqwest::Client;
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix_install() -> BoxedFilter<()> {
    warp::path("shopify_install").boxed()
}

fn path_prefix_confirm() -> BoxedFilter<()> {
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
) -> BoxedFilter<(ConfirmQueryParams, Arc<Config>, Arc<DbConn>, Arc<Client>)> {
    warp::get()
        .and(path_prefix_confirm())
        .and(warp::query::query::<ConfirmQueryParams>())
        .and(with_config(config))
        .and(with_db_conn(db_conn))
        .and(with_reqwest_client(client))
        .boxed()
}
