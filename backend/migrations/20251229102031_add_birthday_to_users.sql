-- Add birthday field to users table
-- Birthday can only be set once and cannot be changed

ALTER TABLE users ADD COLUMN birthday DATE;

-- Index for efficient birthday queries (for the daily job)
CREATE INDEX idx_users_birthday_month_day ON users (
    EXTRACT(MONTH FROM birthday),
    EXTRACT(DAY FROM birthday)
) WHERE birthday IS NOT NULL;
