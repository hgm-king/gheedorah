use log::info;
use warp::{reply, Rejection, Reply};

pub async fn handle_shopify_order(
    order: serde_json::value::Value,
) -> Result<impl Reply, Rejection> {
    info!("{:?}", order);
    Ok(reply::json(&order))
}
