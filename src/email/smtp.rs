use std::env::var;
use crate::prelude::*;
use anyhow::Result;
use serde::Serialize;

use lettre::message::MultiPart;
use lettre::transport::smtp::{authentication::Credentials, client::Tls};
use lettre::{Message, SmtpTransport, Transport};

pub fn env_exists_and_not_empty(env: &str) {
    if var(env).expect(&format!("{} not set!", env)).is_empty() {
        panic!("{} is empty", env)
    }
}

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    [
        "EMAIL_DEFAULT_FROM",
        "EMAIL_SMTP_HOST",
        "EMAIL_SMTP_PORT",
        "EMAIL_SMTP_USERNAME",
        "EMAIL_SMTP_PASSWORD",
    ]
    .iter()
    .for_each(|env| env_exists_and_not_empty(env));
}

#[derive(Debug, Default, Serialize)]
pub struct Email {
    /// Who's sending this.
    #[serde(rename = "From")]
    pub from: String,

    /// Who to send to. Comma-delimited.
    #[serde(rename = "To")]
    pub to: String,

    /// Who to send to. Comma-delimited.
    #[serde(rename = "Subject")]
    pub subject: String,

    /// What to send (plaintext)
    #[serde(rename = "TextBody")]
    pub body: String,

    /// What to send (HTML)
    #[serde(rename = "HtmlBody")]
    pub body_html: String,

    /// Postmark stream to use
    #[serde(rename = "MessageStream")]
    pub postmark_message_stream: String,
}


impl Email {
    /// Send the email. Relies on you ensuring that `EMAIL_DEFAULT_FROM`,
    /// `EMAIL_SMTP_HOST`, `EMAIL_SMTP_USERNAME`, and `EMAIL_SMTP_PASSWORD`
    /// are set in your `.env`.
    pub fn send_via_smtp(&self) -> Result<(), anyhow::Error> {
        let host = var("EMAIL_SMTP_HOST").expect("EMAIL_SMTP_HOST not set!");
        let port = var("EMAIL_SMTP_PORT").expect("EMAIL_SMTP_PORT not set!");
        let username = var("EMAIL_SMTP_USERNAME").expect("EMAIL_SMTP_USERNAME not set!");
        let password = var("EMAIL_SMTP_PASSWORD").expect("EMAIL_SMTP_PASSWORD not set!");
        let reply_to = var("JELLY_SUPPORT_EMAIL")
            .or_else::<anyhow::Error, _>(|_v| Ok(self.from.clone()))
            .unwrap();

        let email = Message::builder()
            .from(self.from.parse()?)
            .reply_to(reply_to.parse()?)
            .to(self.to.parse()?)
            .subject(&self.subject)
            .multipart(MultiPart::alternative_plain_html(
                self.body.clone(),
                self.body_html.clone(),
            ))?;

        let creds = Credentials::new(username, password);

        // Open a remote connection to EMAIL_SMTP_HOST
        let mut mailer_builder = SmtpTransport::relay(&host)?
            .port(port.parse()?)
            .credentials(creds);
        if let Ok(notls) = var("EMAIL_SMTP_NOTLS").and_then(|v| Ok(v == "1" || v == "true")) {
            if notls {
                mailer_builder = mailer_builder.tls(Tls::None);
                info!("Send email with no TLS");
            }
        }

        let mailer = mailer_builder.build();
        mailer.send(&email)?;
        debug!("Mail sent to {} via smtp.", &self.to);

        Ok(())
    }
}