//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Convert config variable error {0}")]
    ConvertConfigVariable(String),

    #[error("Check if account id already exists query error {0}")]
    CheckIfAccountIdAlreadyExistsQuery(String),

    #[error("Check if account change id already exists query error {0}")]
    CheckIfAccountChangeIdAlreadyExistsQuery(String),

    #[error("Account not found error {0}")]
    AccountNotFound(String),

    #[error("Account change not found error {0}")]
    AccountChangeNotFound(String),

    #[error("Delete account change query error {0}")]
    DeleteAccountChangeQuery(String),

    #[error("Delete all account changes query error {0}")]
    DeleteAllAccountChangesQuery(String),

    #[error("Delete all accounts query error {0}")]
    DeleteAllAccountsQuery(String),

    #[error("Delete expired account changes query error {0}")]
    DeleteExpiredAccountChangesQuery(String),

    #[error("Get account from id query error {0}")]
    GetAccountFromIdQuery(String),

    #[error("Get account from id and last change timestamp query error {0}")]
    GetAccountFromIdAndLastChangeTimestampQuery(String),

    #[error("Get account from email query error {0}")]
    GetAccountFromEmail(String),

    #[error("Get account change from change id and account id query error {0}")]
    GetAccountChangeFromChangeIdAndAccountId(String),

    #[error("Read config variable error {0} {1}")]
    ReadConfigVariable(String, String),

    #[error("Create account query error {0}")]
    CreateAccountQuery(String),

    #[error("Update account query error {0}")]
    UpdateAccountQuery(String),

    #[error("Delete account query error {0}")]
    DeleteAccountQuery(String),

    #[error("Get account query error {0}")]
    GetAccountQuery(String),

    #[error("Get account change query error {0}")]
    GetAccountChangeQuery(String),

    #[error("Row not found error {0}")]
    RowNotFound(String),

    #[error("Create account duplicate key error {0}")]
    CreateAccountDuplicateKey(String),

    #[error("Create account change duplicate key error {0}")]
    CreateAccountChangeDuplicateKey(String),

    #[error("Create account change query error {0}")]
    CreateAccountChangeQuery(String),

    #[error("Delete account info change query error {0}")]
    DeleteAccountInfoChangeQuery(String),

    #[error("Create id retry limit exceeded error")]
    CreateIdRetryLimitExceeded(),

    #[error("Plain not equal to encrypted string error")]
    PlainNotEqualToEncryptedString(),

    #[error("Create jwt error {0}")]
    CreateJwt(String),

    #[error("Decode jwt error {0}")]
    DecodeJwt(String),

    #[error("String encrypt error {0}")]
    StringEncrypt(String),

    #[error("Compare plain to encrypted string error {0}")]
    ComparePlainToEncryptedString(String),

    #[error("Send email parse error {0}")]
    SendEmailParse(String),

    #[error("Send email build error {0}")]
    SendEmailBuild(String),

    #[error("Send email connect relay error {0}")]
    SendEmailConnectRelay(String),

    #[error("Send email error {0}")]
    SendEmail(String),

    #[error("Delete expired unverified accounts query error {0}")]
    DeleteExpiredUnverifiedAccountsQuery(String),

    #[error("Delete expired account info changes error {0}")]
    DeleteExpiredAccountInfoChanges(String),
}
