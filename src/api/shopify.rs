#[macro_export]
macro_rules! shopify {
    ($config:expr, $db:expr, $client:expr) => {
        shopify_route::shopify_install($config, $db)
            .and_then(shopify_handler::handle_shopify_installation_request)
            .or(shopify_route::shopify_confirm($config, $db, $client)
                .and_then(shopify_handler::handle_shopify_installation_confirmation))
            .recover(shopify_handler::shopify_handle_rejection)
    };
}
