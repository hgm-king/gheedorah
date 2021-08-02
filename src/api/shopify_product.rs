#[macro_export]
macro_rules! shopify_product {
    () => {
        shopify_product_route::shopify_product()
            .and_then(shopify_product_handler::handle_shopify_product)
            .with(warp::log("shopify_product"))
    };
}
