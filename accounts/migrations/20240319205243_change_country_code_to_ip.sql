-- Purpose: Remove country_code column and add ip column to sessions table.

-- Remove country_code column from sessions table.
ALTER TABLE "sessions"
DROP COLUMN "country_code";

-- Add ip column to sessions table.
ALTER TABLE "sessions"
ADD COLUMN "ip_address" TEXT;

-- Set ip to 'unknown' for all existing sessions.
UPDATE "sessions"
SET "ip_address" = '127.0.0.1'
WHERE "ip_address" IS NULL;

-- Set the ip column to NOT NULL.
ALTER TABLE "sessions"
ALTER COLUMN "ip_address" SET NOT NULL;
