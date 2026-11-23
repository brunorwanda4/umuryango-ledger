-- Migration: 2026112305_projects_loans_future_plans
-- Description: Projects, internal/external loans, and future financial planning goals.

CREATE TYPE project_status AS ENUM ('planning', 'in_progress', 'completed', 'paused');
CREATE TYPE loan_type AS ENUM ('given', 'taken');
CREATE TYPE loan_status AS ENUM ('active', 'repaid', 'defaulted');

CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    family_id UUID NOT NULL REFERENCES families(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    target_amount NUMERIC(18, 4) NOT NULL DEFAULT 0.0000,
    allocated_amount NUMERIC(18, 4) NOT NULL DEFAULT 0.0000,
    status project_status NOT NULL DEFAULT 'planning',
    start_date DATE,
    target_end_date DATE,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS loans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scope financial_scope NOT NULL,
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    counterparty_name VARCHAR(255) NOT NULL,
    principal_amount NUMERIC(18, 4) NOT NULL CHECK (principal_amount > 0),
    balance_remaining NUMERIC(18, 4) NOT NULL CHECK (balance_remaining >= 0),
    interest_rate NUMERIC(5, 2) NOT NULL DEFAULT 0.00,
    due_date DATE,
    loan_type loan_type NOT NULL,
    status loan_status NOT NULL DEFAULT 'active',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS future_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scope financial_scope NOT NULL,
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    estimated_cost NUMERIC(18, 4) NOT NULL DEFAULT 0.0000,
    target_date DATE,
    is_achieved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_projects_family ON projects(family_id);
CREATE INDEX idx_loans_family ON loans(family_id);
CREATE INDEX idx_loans_user ON loans(user_id);
CREATE INDEX idx_future_plans_family ON future_plans(family_id);
