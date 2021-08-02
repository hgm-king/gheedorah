use log::info;
use warp::{reply, Rejection, Reply};

pub async fn handle_shopify_product(
    product: serde_json::value::Value,
) -> Result<impl Reply, Rejection> {
    info!("{:?}", product);
    Ok(reply::json(&product))
}
