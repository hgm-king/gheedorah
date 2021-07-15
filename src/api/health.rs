#[macro_export]
macro_rules! health {
    () => {
        health_route::health()
            .and_then(health_handler::health)
            .with(warp::log("health"))
    };
}
