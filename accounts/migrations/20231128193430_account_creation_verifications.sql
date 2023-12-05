CREATE TABLE "account_creation_verifications" (
    "email" TEXT NOT NULL,
    "handle" TEXT NOT NULL,
    "verification_code" TEXT NOT NULL,
    "verification_code_created_at" TIMESTAMP NOT NULL
)