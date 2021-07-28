use crate::config::Config;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use log::info;
use std::error::Error;
use std::sync::Arc;

pub struct EmailClient {
    mailer: SmtpTransport,
    is_mocking: bool,
}

impl EmailClient {
    pub fn new(is_mocking: bool, config: Arc<Config>) -> Self {
        info!("💌 Setting up Mailer!");
        let mailer = if is_mocking {
            info!("SMTP is mocked!!");
            SmtpTransport::unencrypted_localhost()
        } else {
            let creds = Credentials::new(config.smtp_user.clone(), config.smtp_pass.clone());
            info!("SMTP hosted at: {}", config.smtp_host);
            SmtpTransport::relay(&config.smtp_host)
                .unwrap()
                .credentials(creds)
                .build()
        };

        EmailClient { mailer, is_mocking }
    }

    pub fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(String::from(body))?;
        info!("Sending mail to {}; subject: {}", to, subject);
        self.mailer.send(&email)?;

        Ok(())
    }
}

pub fn generate_email_client(config: Arc<Config>) -> EmailClient {
    EmailClient::new(false, config)
}

pub fn mock_email_client(config: Arc<Config>) -> EmailClient {
    EmailClient::new(true, config)
}
