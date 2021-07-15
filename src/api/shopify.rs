#[macro_export]
macro_rules! shopify {
    ($config:expr, $db:expr, $client:expr) => {
        shopify_route::shopify_install($config, $db)
        .or(shopify_route::shopify_confirm($config, $db, $client))
    };
}
