#[macro_export]
macro_rules! shopify {
    ($config:expr, $db:expr, $client:expr) => {
        shopify_route::shopify_install($config, $db)
            .and_then(shopify_handler::shopify_install)
            .or(shopify_route::shopify_confirm($config, $db, $client)
                .and_then(shopify_handler::is_valid_domain)
                .and_then(shopify_handler::shopify_confirm))
    };
}
