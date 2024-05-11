//! # Sending e-mails using SMTP
//! Can be used to send verification codes, password reset links, etc.

// WIP

// use anyhow::Context;
// use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
// use uuid::Uuid;
// use lazy_static::lazy_static;

// use crate::http::HttpResult;

// lazy_static! {
//     static ref DOMAIN: String = std::env::var("DOMAIN").expect("DOMAIN env variable is not set");
//     static ref SMTP_ADDRESS: String = std::env::var("SMTP_ADDRESS").expect("SMTP_ADDRESS env variable is not set");
//     static ref SMTP_PASSWORD: String = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env variable is not set");
//     static ref MAILER: SmtpTransport = {
//         let creds = Credentials::new(SMTP_ADDRESS.to_string(), SMTP_PASSWORD.to_string());
//         SmtpTransport::relay("smtp.gmail.com")
//             .unwrap()
//             .credentials(creds)
//             .build()
//     };
//     static ref FROM: String = format!("{} <{}>", DOMAIN.to_string(), SMTP_ADDRESS.to_string());
// }