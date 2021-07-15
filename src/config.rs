use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub app_addr: String,
    pub shopify_api_key: String,
    pub shopify_api_secret: String,
    pub shopify_api_uri: String,
    pub tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub db_path: String,
    pub is_mocking: bool,
}

impl Config {
    pub fn new(is_mocking: bool) -> Self {
        dotenv().ok();

        let app_host = env::var("HOST").expect("HOST must be set");
        let app_port = env::var("PORT").expect("PORT must be set");

        let app_addr = format!("{}:{}", app_host, app_port);

        let shopify_api_key = env::var("API_KEY_SHOPIFY").expect("API_KEY_SHOPIFY must be set");
        let shopify_api_secret =
            env::var("API_SECRET_SHOPIFY").expect("API_SECRET_SHOPIFY must be set");

        let tls = env::var("ENABLE_TLS")
            .expect("ENABLE_TLS must be set")
            .parse()
            .expect("ENABLE_TLS must be true or false");

        let cert_path = if tls {
            Some(env::var("CERT_PATH").expect("CERT_PATH must be set"))
        } else {
            None
        };

        let key_path = if tls {
            Some(env::var("KEY_PATH").expect("KEY_PATH must be set"))
        } else {
            None
        };

        let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let shopify_api_uri = String::from("https://");

        Config {
            app_addr,
            shopify_api_key,
            shopify_api_secret,
            shopify_api_uri,
            tls,
            cert_path,
            key_path,
            db_path,
            is_mocking,
        }
    }

    pub fn set_shopify_api_uri(&mut self, uri: String) {
        self.shopify_api_uri = uri;
    }

    pub fn set_shopify_secret_key(&mut self, uri: String) {
        self.shopify_api_secret = uri;
    }
}

pub fn generate_mocking_config() -> Config {
    Config::new(true)
}

pub fn generate_config() -> Config {
    Config::new(false)
}
