CREATE TABLE accounts (
    -------------------------
    -- Account Information --
    -------------------------
    "id" TEXT NOT NULL PRIMARY KEY,
    "picture_id" TEXT,
    "handle" TEXT NOT NULL CONSTRAINT "handle" UNIQUE,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL CONSTRAINT "email" UNIQUE,
    "password" TEXT NOT NULL,
    "group" TEXT NOT NULL,
    "gender" TEXT NOT NULL,
    "country_code" TEXT NOT NULL,
    -------------------------
    -- Account Preferences --
    -------------------------
    "email_is_public" BOOLEAN NOT NULL,
    "gender_is_public" BOOLEAN NOT NULL,
    --------------------------------
    -- Account Creation timestamp --
    --------------------------------
    "created_at" TIMESTAMP NOT NULL,
    ------------------------------
    -- Email Change Verfication --
    ------------------------------
    "original_email_verification_code" TEXT,
    "new_email_verification_code" TEXT,
    "email_verification_codes_created_at" TIMESTAMP,
    ---------------------------------
    -- Passord Change Verification --
    ---------------------------------
    "new_password_verification_code" TEXT,
    "new_password_verification_code_created_at" TIMESTAMP,
    -----------------------------------
    -- Account Deletion Verification --
    -----------------------------------
    "account_deletion_verification_code" TEXT,
    "account_deletion_verification_code_created_at" TIMESTAMP
)