use crate::config::CONFIG;
use crate::error::{EmailError, Error};
use crate::prelude::*;
use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(recipient: &str, subject: &str, body: &str, html: bool) -> Result<()> {
    let sending = f!("{} <{}>", &CONFIG.email_username, &CONFIG.email_address)
        .parse()
        .map_err(|err: AddressError| {
            Error::Email(EmailError::ParseSenderAddress(err.to_string()))
        })?;

    let recieving_address = recipient.parse().map_err(|err: AddressError| {
        Error::Email(EmailError::ParseRecipientAddress(err.to_string()))
    })?;

    let mut email_builder = Message::builder()
        .from(sending)
        .to(recieving_address)
        .subject(subject);

    if html {
        email_builder = email_builder.header(ContentType::TEXT_HTML);
    }

    let email = email_builder
        .body(body.to_string())
        .map_err(|err| Error::Email(EmailError::Build(err.to_string())))?;

    let creds = Credentials::new(
        CONFIG.smtp_username.to_owned(),
        CONFIG.smtp_password.to_owned(),
    );

    let mailer = if CONFIG.smtp_starttls {
        SmtpTransport::starttls_relay(&CONFIG.smtp_relay)
    } else {
        SmtpTransport::relay(&CONFIG.smtp_relay)
    }
    .map_err(|err| Error::Email(EmailError::StartRelayConnection(err.to_string())))?
    .credentials(creds)
    .port(CONFIG.smtp_port)
    .build();

    mailer
        .send(&email)
        .map_err(|err| Error::Email(EmailError::Send(err.to_string())))?;

    Ok(())
}
