use dotenv::dotenv;
use log::info;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub app_addr: String,
    pub shopify_api_key: String,
    pub shopify_api_secret: String,
    pub shopify_api_uri: String,
    pub shopify_graphql_path: String,
    pub shopify_access_scopes: String,
    pub shopify_installation_confirmation_uri: String,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub db_path: String,
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

        // shopify api creds
        let shopify_api_uri = String::from("https://");
        let shopify_api_key = env::var("API_KEY_SHOPIFY").expect("API_KEY_SHOPIFY must be set");
        let shopify_api_secret =
            env::var("API_SECRET_SHOPIFY").expect("API_SECRET_SHOPIFY must be set");

        // our access scopes allow us to do these things
        let shopify_access_scopes =
            String::from("read_products,write_products,read_orders,write_orders");
        // the path to send the user when they confirm the installation of our app
        let shopify_installation_confirmation_uri =
            String::from("https://localhost:3030/shopify/confirm");

        let shopify_graphql_path = String::from("/admin/api/2021-07/graphql.json");

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

        let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Config {
            app_addr,
            shopify_api_key,
            shopify_api_secret,
            shopify_api_uri,
            shopify_graphql_path,
            shopify_access_scopes,
            shopify_installation_confirmation_uri,
            smtp_host,
            smtp_user,
            smtp_pass,
            tls,
            cert_path,
            key_path,
            db_path,
            is_mocking,
        }
    }

    // used for mocks; overrides the shopify api uri with a uri that mockito is listening on
    #[cfg(feature = "mocks")]
    pub fn set_shopify_api_uri(&mut self, uri: String) {
        self.shopify_api_uri = uri;
    }

    // used for mocks; during shopify hmac validation testing, we need to hardcode a
    // result that depends on this value
    #[cfg(feature = "mocks")]
    pub fn set_shopify_secret_key(&mut self, uri: String) {
        self.shopify_api_secret = uri;
    }

    // builds a uri for the given shopify shop; when mocking, we spit back the mockito uri
    pub fn get_shopify_api_uri(&self, shop: String) -> String {
        if self.is_mocking {
            self.shopify_api_uri.clone()
        } else {
            format!("{}{}", self.shopify_api_uri.clone(), shop)
        }
    }

    // used for mocks; overrides the shopify graphql uri with a uri that mockito is listening on
    #[cfg(feature = "mocks")]
    pub fn set_shopify_graphql_path(&mut self, uri: String) {
        self.shopify_graphql_path = uri;
    }

    pub fn get_shopify_graphql_path(&self, shop: String) -> String {
        if self.is_mocking {
            self.shopify_graphql_path.clone()
        } else {
            format!("https://{}{}", shop, self.shopify_graphql_path.clone())
        }
    }
}

#[cfg(feature = "mocks")]
pub fn generate_mocking_config() -> Config {
    Config::new(true)
}

pub fn generate_config() -> Config {
    Config::new(false)
}
