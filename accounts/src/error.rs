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
pub enum S3Error {
    #[error("Failed to instantiate S3 bucket")]
    InstantiateBucket(String),

    #[error("Failed to head object: {0}")]
    HeadObject(String),

    #[error("Bucket not found")]
    BucketNotFound,

    #[error("Failed to put object in bucket: {0}")]
    PutObject(String),
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to fetch row")]
    FetchOne(String),

    #[error("Row not found")]
    RowNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Email(EmailError),

    #[error(transparent)]
    Encryption(EncryptionError),

    #[error(transparent)]
    Token(TokenError),

    #[error(transparent)]
    S3(S3Error),

    #[error(transparent)]
    Database(DatabaseError),

    #[error(transparent)]
    Regex(regex::Error),
}
