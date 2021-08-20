use dotenv::dotenv;
use log::info;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub app_addr: String,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub db_path: String,
    pub shopify: ShopifyConfig,
    pub is_mocking: bool,
}

impl Config {
    pub fn new(is_mocking: bool) -> Self {
        info!("🤖 Configuring the application!");
        dotenv().ok();

        // app fields
        let app_host = env::var("HOST").expect("HOST must be set");
        let app_port = env::var("PORT").expect("PORT must be set");
        let app_addr = format!("{}:{}", app_host, app_port);

        // mailer variables
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");

        // prepare tls if necessary
        let tls = env::var("ENABLE_TLS")
            .expect("ENABLE_TLS must be set")
            .parse()
            .expect("ENABLE_TLS must be true or false");

        let cert_path;
        let key_path;
        if tls {
            cert_path = Some(env::var("CERT_PATH").expect("CERT_PATH must be set"));
            key_path = Some(env::var("KEY_PATH").expect("KEY_PATH must be set"));
        } else {
            cert_path = None;
            key_path = None;
        }

        // url to connect to the database
        let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let shopify = ShopifyConfig::new(is_mocking);

        Config {
            app_addr,
            smtp_host,
            smtp_user,
            smtp_pass,
            tls,
            cert_path,
            key_path,
            db_path,
            shopify,
            is_mocking,
        }
    }
}

#[derive(Clone)]
pub struct ShopifyConfig {
    pub is_mocking: bool,
    pub api_key: String,
    pub api_secret: String,
    pub api_domain: Option<String>,
    pub api_path: String,
    pub install_redirect_path: String,
    pub graphql_domain: Option<String>,
    pub graphql_path: String,
    pub access_scopes: String,
    pub webhook_domain: String,
    pub install_domain: String,
    pub install_path: String,
    pub confirm_path: String,
    pub order_webhook_path: String,
    pub product_webhook_path: String,
}

impl ShopifyConfig {
    pub fn new(is_mocking: bool) -> Self {
        // shopify api creds
        let api_key = env::var("API_KEY_SHOPIFY").expect("API_KEY_SHOPIFY must be set");
        let api_secret = env::var("API_SECRET_SHOPIFY").expect("API_SECRET_SHOPIFY must be set");

        // our access scopes allow us to do these things
        let access_scopes = String::from("read_products,write_products,read_orders,write_orders");

        // the domains that our service will be listening on
        let webhook_domain =
            env::var("SHOPIFY_WEBHOOK_DOMAIN").expect("SHOPIFY_WEBHOOK_DOMAIN must be set");
        let install_domain =
            env::var("SHOPIFY_INSTALL_DOMAIN").expect("SHOPIFY_INSTALL_DOMAIN must be set");

        // our paths that our app will be using to listen to specifics
        let install_path = String::from("/shopify/install");
        let confirm_path = String::from("/shopify/confirm");
        let order_webhook_path = String::from("/shopify/order");
        let product_webhook_path = String::from("/shopify/product");

        // domains and paths to any external Shopify services that we will use.
        // these two need to be none, they will be set by a mockito url if we are mocking for tests
        let api_domain = None;
        let graphql_domain = None;
        let api_path = String::from("/admin/oauth/access_token");
        let graphql_path = String::from("/admin/api/2021-07/graphql.json");
        let install_redirect_path = String::from("/admin/oauth/authorize");

        ShopifyConfig {
            is_mocking,
            api_key,
            api_secret,
            api_domain,
            api_path,
            install_redirect_path,
            graphql_domain,
            graphql_path,
            access_scopes,
            webhook_domain,
            install_domain,
            install_path,
            confirm_path,
            order_webhook_path,
            product_webhook_path,
        }
    }

    // used for mocks; during shopify hmac validation testing, we need to hardcode a
    // result that depends on this value
    #[cfg(feature = "mocks")]
    pub fn set_mocked_server_uri(&mut self, domain: String) {
        self.set_api_domain(domain.clone());
        self.set_graphql_domain(domain);
    }

    // used for mocks; during shopify hmac validation testing, we need to hardcode a
    // result that depends on this value
    #[cfg(feature = "mocks")]
    pub fn set_secret_key(&mut self, key: String) {
        self.api_secret = key;
    }

    // used for mocks; overrides the shopify api domain with a domain that mockito is listening on
    #[cfg(feature = "mocks")]
    pub fn set_api_domain(&mut self, domain: String) {
        self.api_domain = Some(domain);
    }

    // builds a uri for the given shopify shop; when mocking, we spit back the mockito domain
    pub fn get_api_url(&self, shop: String) -> String {
        if self.is_mocking {
            format!(
                "{}{}",
                self.api_domain.as_ref().unwrap().clone(),
                self.api_path.clone()
            )
        } else {
            format!("https://{}{}", shop, self.api_path.clone())
        }
    }

    // used for mocks; overrides the shopify graphql domain with a domain that mockito is listening on
    #[cfg(feature = "mocks")]
    pub fn set_graphql_domain(&mut self, domain: String) {
        self.graphql_domain = Some(domain);
    }

    // builds a uri for the shop's graphql api, when mocking we spit back the mockito domain
    pub fn get_graphql_url(&self, shop: String) -> String {
        if self.is_mocking {
            format!(
                "{}{}",
                self.graphql_domain.as_ref().unwrap().clone(),
                self.graphql_path.clone()
            )
        } else {
            format!("https://{}{}", shop, self.graphql_path.clone())
        }
    }

    pub fn get_order_webhook_uri(&self) -> String {
        format!(
            "{}{}",
            self.webhook_domain.clone(),
            self.order_webhook_path.clone()
        )
    }

    pub fn get_product_webhook_uri(&self) -> String {
        format!(
            "{}{}",
            self.webhook_domain.clone(),
            self.product_webhook_path.clone()
        )
    }

    pub fn get_install_uri(&self) -> String {
        format!(
            "{}{}",
            self.install_domain.clone(),
            self.install_path.clone()
        )
    }

    pub fn get_confirm_uri(&self) -> String {
        format!(
            "{}{}",
            self.install_domain.clone(),
            self.confirm_path.clone()
        )
    }
}

#[cfg(feature = "mocks")]
pub fn generate_mocking_config() -> Config {
    Config::new(true)
}

pub fn generate_config() -> Config {
    Config::new(false)
}
