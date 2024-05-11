//! # Sending e-mails using SMTP
//! Can be used to send verification codes, password reset links, etc.

// WIP

// use anyhow::Context;
// use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
// use uuid::Uuid;
// use once_cell::sync::Lazy;
// use crate::http::HttpResult;

// static DOMAIN: Lazy<String> = Lazy::new(|| std::env::var("DOMAIN").expect("DOMAIN env variable is not set"));
// static SMTP_ADDRESS: Lazy<String> = Lazy::new(|| std::env::var("SMTP_ADDRESS").expect("SMTP_ADDRESS env variable is not set"));
// static SMTP_PASSWORD: Lazy<String> = Lazy::new(|| std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable is not set"));
// static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
//     let creds = Credentials::new(SMTP_ADDRESS.to_string(), SMTP_PASSWORD.to_string());
//     SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build()
// });
// static FROM: Lazy<String> = Lazy::new(|| format!("{} <{}>", DOMAIN.to_string(), SMTP_ADDRESS.to_string()));