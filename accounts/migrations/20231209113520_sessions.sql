CREATE TABLE "sessions" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "account_id" TEXT NOT NULL REFERENCES "accounts" ("id") ON DELETE CASCADE,
    "device" TEXT NOT NULL UNIQUE,
    "device_type" TEXT NOT NULL,
    "expire_date" TIMESTAMP NOT NULL,
    "created_at" TIMESTAMP NOT NULL
)