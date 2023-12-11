CREATE TABLE accounts (
    -------------------------
    -- Account Information --
    -------------------------
    "id" TEXT NOT NULL PRIMARY KEY,
    "picture_id" TEXT NOT NULL,
    "handle" TEXT NOT NULL CONSTRAINT "handle" UNIQUE,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL CONSTRAINT "email" UNIQUE,
    "password" TEXT NOT NULL,
    "group" TEXT NOT NULL,
    "gender" TEXT NOT NULL,
    -------------------------
    -- Account Preferences --
    -------------------------
    "theme" TEXT NOT NULL,
    "language" TEXT NOT NULL,
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
    "new_password_verification_code_created_at" TIMESTAMP
)