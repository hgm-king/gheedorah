use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path!("shopify" / "product").boxed()
}

pub fn shopify_product() -> BoxedFilter<(serde_json::value::Value,)> {
    warp::post()
        .and(path_prefix())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .boxed()
}
