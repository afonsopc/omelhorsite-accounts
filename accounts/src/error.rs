// make compatibile with anyhow

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Failed to build email")]
    Build(String),
    #[error("Failed to parse sender address")]
    ParseSenderAddress(String),
    #[error("Failed to parse recipient address")]
    ParseRecipientAddress(String),
    #[error("Failed to send email")]
    StartRelayConnection(String),
    #[error("Failed to send email")]
    Send(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to connect to database")]
    Email(EmailError),
}
