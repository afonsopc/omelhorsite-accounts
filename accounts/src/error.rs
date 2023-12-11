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
pub enum EncryptionError {
    #[error("Failed to encrypt string")]
    EncryptString(String),

    #[error("Failed to compare plain to encrypted string")]
    ComparePlainToEncryptedString(String),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Failed to create Token")]
    CreateToken(String),

    #[error("Failed to decode Token")]
    DecodeToken(String),

    #[error("Missing Authorization Header")]
    MissingAuthorizationHeader(String),

    #[error("Invalid Token")]
    InvalidToken,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Email(EmailError),
    #[error(transparent)]
    Encryption(EncryptionError),
    #[error(transparent)]
    Token(TokenError),
}
