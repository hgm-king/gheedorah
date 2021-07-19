use crate::HealthResponse;
use warp::{Rejection, Reply, reply};

pub async fn health() -> Result<impl Reply, Rejection> {
    Ok(reply::json(&HealthResponse::new(String::from("OK"))))
}
