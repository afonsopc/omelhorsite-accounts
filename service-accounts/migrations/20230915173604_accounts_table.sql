CREATE TABLE accounts (
    "account_id"            TEXT NOT NULL UNIQUE,
    "username"                  TEXT NOT NULL,
    "email"                 TEXT NOT NULL UNIQUE,
    "password"              TEXT NOT NULL,
    "verified"              BOOLEAN NOT NULL,
    "last_change_timestamp" TEXT NOT NULL,
    "creation_timestamp"    TEXT NOT NULL
);