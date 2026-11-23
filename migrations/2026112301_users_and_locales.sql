-- Migration: 2026112301_users_and_locales
-- Description: Create base user accounts, preferred locales, and authentication primitives.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE app_locale AS ENUM ('en', 'rw');

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE,
    phone_number VARCHAR(32) UNIQUE,
    full_name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    preferred_locale app_locale NOT NULL DEFAULT 'rw',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_phone ON users(phone_number);
