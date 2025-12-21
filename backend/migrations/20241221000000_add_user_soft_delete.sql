-- Add soft delete support for users
-- This allows keeping booking history while marking users as deleted

ALTER TABLE users ADD COLUMN deleted_at TIMESTAMPTZ;

CREATE INDEX idx_users_deleted ON users(deleted_at) WHERE deleted_at IS NULL;

-- Add comment for documentation
COMMENT ON COLUMN users.deleted_at IS 'When set, indicates the user has been deleted. PII should be cleared.';
