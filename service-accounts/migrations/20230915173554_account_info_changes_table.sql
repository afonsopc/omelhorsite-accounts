CREATE TABLE account_changes (
    "account_change_id"      TEXT NOT NULL UNIQUE,
    "account_id"             TEXT NOT NULL UNIQUE,
    "name"                   TEXT,
    "email"                  TEXT UNIQUE,
    "password"               TEXT,
    "verified"               BOOLEAN,
    "step"                   SMALLINT,
    "creation_timestamp"     TEXT NOT NULL
);