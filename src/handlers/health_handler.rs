use crate::HealthResponse;
use warp::{reply, Rejection, Reply};

pub async fn health() -> Result<impl Reply, Rejection> {
    Ok(reply::json(&HealthResponse::new(String::from("OK"))))
}
