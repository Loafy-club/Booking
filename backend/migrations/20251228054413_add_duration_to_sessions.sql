-- Add end_time column to sessions table
-- This allows us to calculate duration from start time (time) and end time

ALTER TABLE sessions
ADD COLUMN end_time TIME;

-- Add a comment for clarity
COMMENT ON COLUMN sessions.end_time IS 'Session end time, used with time (start) to calculate duration';
