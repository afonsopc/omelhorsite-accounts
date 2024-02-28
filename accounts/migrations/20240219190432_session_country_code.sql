-- Purpose: Add country_code column to sessions table.
ALTER TABLE "sessions"
ADD COLUMN "country_code" TEXT;

-- Set country_code to 'unknown' for all existing sessions.
UPDATE "sessions"
SET "country_code" = 'unknown'
WHERE "country_code" IS NULL;

-- Set the country_code column to NOT NULL.
ALTER TABLE "sessions"
ALTER COLUMN "country_code" SET NOT NULL;