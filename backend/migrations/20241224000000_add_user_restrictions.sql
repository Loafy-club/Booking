-- Add restriction/suspension fields to users table
ALTER TABLE users
    ADD COLUMN suspended_at TIMESTAMPTZ,
    ADD COLUMN suspended_until TIMESTAMPTZ,
    ADD COLUMN suspension_reason TEXT,
    ADD COLUMN suspended_by UUID REFERENCES users(id);

-- Index for efficient filtering of suspended users
CREATE INDEX idx_users_suspended ON users(suspended_at) WHERE suspended_at IS NOT NULL;

-- Comments for documentation
COMMENT ON COLUMN users.suspended_at IS 'When set, indicates the user is currently suspended. NULL means active.';
COMMENT ON COLUMN users.suspended_until IS 'Optional expiration date. NULL means indefinite suspension.';
COMMENT ON COLUMN users.suspension_reason IS 'Required reason for suspension, visible to the user.';
COMMENT ON COLUMN users.suspended_by IS 'Admin user who created the suspension.';
