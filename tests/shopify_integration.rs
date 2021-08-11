mod shopify_integration_tests {

    use diesel::prelude::*;
    use mockito::mock;
    use mocktopus::mocking::*;
    use sidecar::{
        config::Config,
        db_conn::DbConn,
        db_test_url,
        handlers::shopify_handler,
        models::shopify_integration::{
            read_by_shop, read_by_shop_and_nonce, NewShopifyIntegration,
        },
        routes::shopify_route,
        schema::shopify_integrations,
        utils::gen_uuid,
    };
    use std::sync::Arc;
    use warp::{self, Filter};

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(shopify_integrations::table)
            .execute(conn)
            .unwrap();
    }

    #[tokio::test]
    async fn it_inserts_on_shopify_installation() {
        let shop_name = "bestbudz.myshopify.com";
        let nonce = "some-nonce";

        // setup context
        let config = Arc::new(Config::new(false));
        let db_conn = Arc::new(DbConn::new(&db_test_url()));
        // setup filters
        let shopify = shopify_route::shopify_install(config.clone(), db_conn.clone())
            .and_then(shopify_handler::handle_shopify_installation_request);
        // prep mocks
        gen_uuid.mock_safe(move || MockResult::Return(nonce.to_string()));

        // send the request to our api,
        // hopefully sending back a redirect and saving an instance in the db
        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/install\
                ?hmac=00a329c0648769a73afac7f9381e08fb43dbea72\
                &shop={}\
                &timestamp=1623154978",
                shop_name
            ))
            .reply(&shopify)
            .await;

        // assertions
        assert_eq!(res.status(), 301);
        let shopify_integration = read_by_shop_and_nonce(
            &db_conn.get_conn(),
            shop_name.to_string(),
            nonce.to_string(),
        )
        .unwrap();
        assert!(0 < shopify_integration.len());

        let my_shopify_integration = shopify_integration.iter().find(|&x| x.shop == shop_name);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        cleanup_table(&db_conn.get_conn());
    }

    #[tokio::test]
    async fn it_follows_shopify_confirm_flow() {
        let shop_name = "shop.myshopify.com";
        let nonce = "89793";
        let access_token = "f85632530bf277ec9ac6f649fc327f17";

        // setup context
        let test_db_url = db_test_url();
        let mut config = Config::new(true);
        config.set_mocked_server_uri(mockito::server_url());
        config.set_shopify_secret_key(String::from("hush"));
        let arc_config = Arc::new(config);
        let db_conn = Arc::new(DbConn::new(&test_db_url));
        let client = Arc::new(reqwest::Client::new());
        // setup filters
        let shopify =
            shopify_route::shopify_confirm(arc_config.clone(), db_conn.clone(), client.clone())
                .and_then(shopify_handler::handle_shopify_installation_confirmation);

        // make sure db is in proper config
        let new_shopify_integration =
            NewShopifyIntegration::new(shop_name.to_string(), nonce.to_string());
        new_shopify_integration.insert(&db_conn.get_conn());
        // prep mocks
        let _m = mock(
            "POST",
            mockito::Matcher::Exact(arc_config.clone().shopify_api_path.clone()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&format!(
            "{{\"access_token\": \"{}\",\"scope\": \"write_orders,read_customers\"}}",
            access_token
        ))
        .create();

        let _m2 = mock(
            "POST",
            mockito::Matcher::Exact(arc_config.clone().shopify_graphql_path.clone()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        //.with_body(include_str!("stubbed-product-create.json"))
        .with_body("{\"data\":{\"productCreate\":{\"product\":{\"id\":\"gid:\\/\\/shopify\\/Product\\/6887105790141\"}}},\"extensions\":{\"cost\":{\"requestedQueryCost\":10,\"actualQueryCost\":10,\"throttleStatus\":{\"maximumAvailable\":1000.0,\"currentlyAvailable\":990,\"restoreRate\":50.0}}}}")
        .create();

        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/confirm\
                    ?code=314159\
                    &hmac=00d39b4e40556ad1f8c8a5c673975e62abc8e0f2574d99a1934e2e881350a710\
                    &host=26535\
                    &shop={}\
                    &state={}\
                    &timestamp=1337178173",
                shop_name, nonce
            ))
            .reply(&shopify)
            .await;

        // assertions
        assert_eq!(res.status(), 301);
        let shopify_integrations =
            read_by_shop(&db_conn.get_conn(), shop_name.to_string()).unwrap();

        assert_eq!(1, shopify_integrations.len());
        let my_shopify_integration = shopify_integrations.iter().find(|x| x.shop == shop_name);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        assert_eq!(
            my_shopify_integration
                .unwrap()
                .access_token
                .as_ref()
                .unwrap(),
            access_token
        );

        cleanup_table(&db_conn.get_conn());
    }

    #[tokio::test]
    async fn it_runs_the_entire_flow() {
        let shop_name = "shop.myshopify.com";
        let nonce = "89793";
        let hmac = "00d39b4e40556ad1f8c8a5c673975e62abc8e0f2574d99a1934e2e881350a710";
        let access_token = "f85632530bf277ec9ac6f649fc327f17";

        // setup context
        let mut config = Config::new(true);
        config.set_mocked_server_uri(mockito::server_url());
        config.set_shopify_secret_key(String::from("hush"));
        let arc_config = Arc::new(config);
        let db_conn = Arc::new(DbConn::new(&db_test_url()));
        let client = Arc::new(reqwest::Client::new());
        // setup filters
        let shopify = shopify_route::shopify_install(arc_config.clone(), db_conn.clone())
            .and_then(shopify_handler::handle_shopify_installation_request)
            .or(shopify_route::shopify_confirm(
                arc_config.clone(),
                db_conn.clone(),
                client.clone(),
            )
            .and_then(shopify_handler::handle_shopify_installation_confirmation));

        // prep mocks
        gen_uuid.mock_safe(move || MockResult::Return(nonce.to_string()));
        let _m = mock(
            "POST",
            mockito::Matcher::Exact(arc_config.clone().shopify_api_path.clone()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&format!(
            "{{\"access_token\": \"{}\",\"scope\": \"write_orders,read_customers\"}}",
            access_token
        ))
        .create();

        let _m2 = mock(
            "POST",
            mockito::Matcher::Exact(arc_config.clone().shopify_graphql_path.clone()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        //.with_body(include_str!("stubbed-product-create.json"))
        .with_body("{\"data\":{\"productCreate\":{\"product\":{\"id\":\"gid:\\/\\/shopify\\/Product\\/6887105790141\"}}},\"extensions\":{\"cost\":{\"requestedQueryCost\":10,\"actualQueryCost\":10,\"throttleStatus\":{\"maximumAvailable\":1000.0,\"currentlyAvailable\":990,\"restoreRate\":50.0}}}}")
        .create();

        // first send off message to install the app
        let install_res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/install\
                ?hmac={}\
                &shop={}\
                &timestamp=1623154978",
                hmac, shop_name
            ))
            .reply(&shopify)
            .await;

        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/confirm\
                    ?code=314159\
                    &hmac={}\
                    &host=26535\
                    &shop={}\
                    &state={}\
                    &timestamp=1337178173",
                hmac, shop_name, nonce
            ))
            .reply(&shopify)
            .await;

        // assertions
        assert_eq!(install_res.status(), 301);
        assert_eq!(res.status(), 301);

        cleanup_table(&db_conn.get_conn());
    }

    #[tokio::test]
    async fn it_handles_a_double_install_scenario() {
        let shop_name = "shop.myshopify.com";
        let nonce = "89793";
        let hmac = "00d39b4e40556ad1f8c8a5c673975e62abc8e0f2574d99a1934e2e881350a710";
        let access_token = "f85632530bf277ec9ac6f649fc327f17";

        // setup context
        let mut config = Config::new(true);
        config.set_mocked_server_uri(mockito::server_url());
        config.set_shopify_secret_key(String::from("hush"));
        let arc_config = Arc::new(config);
        let db_conn = Arc::new(DbConn::new(&db_test_url()));
        let client = Arc::new(reqwest::Client::new());
        // setup filters
        let shopify = shopify_route::shopify_install(arc_config.clone(), db_conn.clone())
            .and_then(shopify_handler::handle_shopify_installation_request)
            .or(shopify_route::shopify_confirm(
                arc_config.clone(),
                db_conn.clone(),
                client.clone(),
            )
            .and_then(shopify_handler::handle_shopify_installation_confirmation));

        // first send off message to install the app, but deny the confirm
        let _res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/install?hmac={}&shop={}&timestamp=1337178173",
                hmac, shop_name
            ))
            .reply(&shopify)
            .await;

        // prep mocks
        gen_uuid.mock_safe(move || MockResult::Return(nonce.to_string()));
        let _m = mock(
            "POST",
            mockito::Matcher::Exact(arc_config.clone().shopify_api_path.clone()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&format!(
            "{{\"access_token\": \"{}\",\"scope\": \"write_orders,read_customers\"}}",
            access_token
        ))
        .create();

        let _m2 = mock(
            "POST",
            mockito::Matcher::Exact(arc_config.clone().shopify_graphql_path.clone()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        //.with_body(include_str!("stubbed-product-create.json"))
        .with_body("{\"data\":{\"productCreate\":{\"product\":{\"id\":\"gid:\\/\\/shopify\\/Product\\/6887105790141\"}}},\"extensions\":{\"cost\":{\"requestedQueryCost\":10,\"actualQueryCost\":10,\"throttleStatus\":{\"maximumAvailable\":1000.0,\"currentlyAvailable\":990,\"restoreRate\":50.0}}}}")
        .create();

        // the user clicks the request button again
        let _res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/install?hmac={}&shop={}&timestamp=1337178173",
                hmac, shop_name
            ))
            .reply(&shopify)
            .await;

        // and goes through and installs
        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/confirm\
                    ?code=314159\
                    &hmac={}\
                    &host=26535\
                    &shop={}\
                    &state={}\
                    &timestamp=1337178173",
                hmac, shop_name, nonce
            ))
            .reply(&shopify)
            .await;

        // assertions
        assert_eq!(res.status(), 301);
        let shopify_integration = read_by_shop_and_nonce(
            &db_conn.get_conn(),
            shop_name.to_string(),
            nonce.to_string(),
        )
        .unwrap();
        assert!(0 < shopify_integration.len());
        let my_shopify_integration = shopify_integration.iter().find(|&x| x.shop == shop_name);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        assert!(my_shopify_integration.unwrap().access_token.is_some());

        cleanup_table(&db_conn.get_conn());
    }
}
