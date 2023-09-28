use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::prelude::*;

use super::config;

pub async fn send_email(
    recieving_adress: &str,
    subject: &String,
    body_html: &String,
) -> Result<()> {
    let app_config = config::AppConfig::load_from_env().unwrap();
    let email_username = app_config.name;
    let email_address = app_config.email_address;
    let email_password = app_config.email_password;
    let smtp_relay = app_config.smtp_relay;

    let sending: Mailbox = match f!("{email_username} <{email_address}>").parse() {
        Ok(value) => value,
        Err(err) => return Err(Error::SendEmailParse(err.to_string())),
    };
    let recieving: Mailbox = match recieving_adress.parse() {
        Ok(value) => value,
        Err(err) => return Err(Error::SendEmailParse(err.to_string())),
    };

    let email = match Message::builder()
        .from(sending)
        .to(recieving)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body_html.to_owned())
    {
        Ok(value) => value,
        Err(err) => return Err(Error::SendEmailBuild(err.to_string())),
    };

    let credentials = Credentials::new(email_address, email_password);

    let mailer = match SmtpTransport::relay(&smtp_relay) {
        Ok(value) => value,
        Err(err) => return Err(Error::SendEmailConnectRelay(err.to_string())),
    }
    .credentials(credentials)
    .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::SendEmail(err.to_string())),
    }
}

pub async fn send_confirmation_email(
    recieving_adress: &str,
    subject: &String,
    title_message: &str,
    confirmation_code: &str,
) -> Result<()> {
    let app_config = config::AppConfig::load_from_env().unwrap();
    let name = app_config.name;
    let confirmation_email_body = app_config.confirmation_email_body;
    let email_name_placeholder = app_config.email_name_placeholder;
    let confirmation_email_title_message_placeholder =
        app_config.confirmation_email_title_message_placeholder;
    let confirmation_email_confirmation_code_placeholder =
        app_config.confirmation_email_confirmation_code_placeholder;

    let body = confirmation_email_body
        .replace(&confirmation_email_title_message_placeholder, title_message)
        .replace(&email_name_placeholder, &name)
        .replace(
            &confirmation_email_confirmation_code_placeholder,
            confirmation_code,
        );

    send_email(recieving_adress, subject, &body).await?;

    Ok(())
}
