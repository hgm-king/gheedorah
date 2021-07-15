use crate::HealthResponse;
use warp::{Rejection, Reply};

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&HealthResponse::new(String::from("OK"))))
}
