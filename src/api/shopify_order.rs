#[macro_export]
macro_rules! shopify_order {
    () => {
        shopify_order_route::shopify_order()
            .and_then(shopify_order_handler::handle_shopify_order)
            .with(warp::log("shopify_order"))
    };
}
