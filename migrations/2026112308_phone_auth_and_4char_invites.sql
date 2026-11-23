-- Migration: 2026112308_phone_auth_and_4char_invites
-- Description: Phone-based authentication, gender, 4-character invite codes, and pre-invite by phone number.

-- 1. Update users table: rename phone_number to phone or add phone, add gender, ensure types and constraints
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'users' AND column_name = 'phone_number'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'users' AND column_name = 'phone'
    ) THEN
        ALTER TABLE users RENAME COLUMN phone_number TO phone;
    END IF;
END $$;

ALTER TABLE users ADD COLUMN IF NOT EXISTS phone VARCHAR(32);
ALTER TABLE users ADD COLUMN IF NOT EXISTS gender VARCHAR(16) NOT NULL DEFAULT 'OTHER';
ALTER TABLE users ALTER COLUMN full_name TYPE TEXT;
ALTER TABLE users ALTER COLUMN password_hash TYPE TEXT;

-- Provide placeholder phone for any legacy accounts with NULL phone before setting NOT NULL
UPDATE users SET phone = '+25078' || LPAD(FLOOR(RANDOM() * 10000000)::TEXT, 7, '0') WHERE phone IS NULL;

ALTER TABLE users ALTER COLUMN phone SET NOT NULL;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'uq_users_phone'
    ) THEN
        BEGIN
            ALTER TABLE users ADD CONSTRAINT uq_users_phone UNIQUE (phone);
        EXCEPTION
            WHEN duplicate_table OR duplicate_object THEN NULL;
        END;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_users_phone_clean ON users(phone);

-- 2. Update families: 4-character uppercase invite_code
-- Backfill existing rows with 4-character codes if null or length != 4
UPDATE families
SET invite_code = UPPER(SUBSTRING(MD5(id::text || clock_timestamp()::text) FROM 1 FOR 4))
WHERE invite_code IS NULL OR LENGTH(TRIM(invite_code)) <> 4;

ALTER TABLE families ALTER COLUMN invite_code TYPE CHAR(4);
ALTER TABLE families ALTER COLUMN invite_code SET NOT NULL;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'uq_families_invite_code'
    ) THEN
        BEGIN
            ALTER TABLE families ADD CONSTRAINT uq_families_invite_code UNIQUE (invite_code);
        EXCEPTION
            WHEN duplicate_table OR duplicate_object THEN NULL;
        END;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_families_invite_code_4char ON families(invite_code);

-- 3. Create family_phone_invites table for pre-invites by phone number
CREATE TABLE IF NOT EXISTS family_phone_invites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    family_id UUID NOT NULL REFERENCES families(id) ON DELETE CASCADE,
    invited_phone VARCHAR(32) NOT NULL,
    invited_by_user_id UUID NOT NULL REFERENCES users(id),
    role VARCHAR(32) NOT NULL DEFAULT 'CUSTODIAN',
    status VARCHAR(16) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_family_phone UNIQUE (family_id, invited_phone)
);

CREATE INDEX IF NOT EXISTS idx_family_phone_invites_phone ON family_phone_invites(invited_phone, status);
CREATE INDEX IF NOT EXISTS idx_family_phone_invites_family ON family_phone_invites(family_id);
