mod shopify_integration_tests {

    use diesel::prelude::*;
    use dotenv::dotenv;
    use mockito::mock;
    use mocktopus::mocking::*;
    use sidecar::{
        config::Config,
        db_conn::DbConn,
        db_test_url,
        handlers::shopify_handler,
        models::shopify_integration::{
            create, read, read_by_shop, read_by_shop_and_nonce, NewShopifyIntegration,
            ShopifyIntegration,
        },
        routes::shopify_route,
        schema::shopify_integrations,
        utils::gen_uuid,
        AccessTokenResponse,
    };
    use std::sync::Arc;
    use uuid::Uuid;
    use warp::{self, Filter};

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(shopify_integrations::table)
            .execute(conn)
            .unwrap();
    }

    #[tokio::test]
    async fn it_inserts_on_shopify_installation() {
        let config = Arc::new(Config::new(false));
        let db_conn = Arc::new(DbConn::new(&db_test_url()));
        let client = Arc::new(reqwest::Client::new());

        let shopify = shopify_route::shopify_install(config.clone(), db_conn.clone())
            .and_then(shopify_handler::handle_shopify_installation_request);

        let shop_name = "bestbudz.myshopify.com";
        let nonce = "some-nonce";

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
        let test_db_url = db_test_url();

        let mut config = Config::new(true);
        config.set_shopify_api_uri(mockito::server_url());
        config.set_shopify_secret_key(String::from("hush"));
        let arc_config = Arc::new(config);

        let db_conn = Arc::new(DbConn::new(&test_db_url));
        let client = Arc::new(reqwest::Client::new());
        let shopify =
            shopify_route::shopify_confirm(arc_config.clone(), db_conn.clone(), client.clone())
                .and_then(shopify_handler::handle_shopify_installation_confirmation);

        let shop_name = "some-shop.myshopify.com";
        let nonce = "0.6784241404160823";
        let access_token = "f85632530bf277ec9ac6f649fc327f17";

        let new_shopify_integration =
            NewShopifyIntegration::new(shop_name.to_string(), nonce.to_string());
        new_shopify_integration.insert(&db_conn.get_conn());

        let _m = mock("POST", "/admin/oauth/access_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&format!(
                "{{\"access_token\": \"{}\",\"scope\": \"write_orders,read_customers\"}}",
                access_token
            ))
            .create();

        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify/confirm\
                    ?code=0907a61c0c8d55e99db179b68161bc00\
                    &hmac=700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf\
                    &host=YmRyb2NrZXRzdG9yZS5teXNob3BpZnkuY29tL2FkbWlu\
                    &shop={}\
                    &state={}\
                    &timestamp=1337178173",
                shop_name, nonce
            ))
            .reply(&shopify)
            .await;

        let shopify_integrations = read_by_shop(&db_conn.get_conn(), shop_name.to_string()).unwrap();

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
}
