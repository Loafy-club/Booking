-- Phase 1: Initial Schema for Loafy Club Booking Platform
-- Tables: roles, users, sessions, bookings, config

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default roles
INSERT INTO roles (name) VALUES ('user'), ('organizer'), ('admin'), ('moderator');

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255),
    avatar_url TEXT,
    phone VARCHAR(50),
    role_id UUID REFERENCES roles(id) NOT NULL,
    auth_provider VARCHAR(50) NOT NULL,
    auth_provider_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role_id);

-- Global configuration table
CREATE TABLE config (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(100) UNIQUE NOT NULL,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default configuration values
INSERT INTO config (key, value, description) VALUES
    ('subscriber_early_access_hours', '168', 'Early access hours for subscribers (default: 7 days)'),
    ('drop_in_price_vnd', '100000', 'Base price for drop-in sessions in VND'),
    ('drop_in_cancellation_hours', '48', 'Cancellation deadline for drop-ins in hours'),
    ('subscriber_cancellation_hours', '24', 'Cancellation deadline for subscribers in hours'),
    ('subscriber_out_of_ticket_discount_percent', '10', 'Discount percentage when subscriber is out of tickets'),
    ('payment_deadline_minutes', '30', 'Time to complete payment in minutes'),
    ('waitlist_exclusive_hours', '1', 'Hours before next person is notified in waitlist'),
    ('max_players_per_court', '6', 'Default maximum players per court'),
    ('birthday_account_age_days', '30', 'Minimum account age in days for birthday ticket'),
    ('referral_bonus_tickets', '1', 'Number of tickets given on referral'),
    ('birthday_bonus_tickets', '1', 'Number of tickets given on birthday');

-- Sessions table
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organizer_id UUID REFERENCES users(id) NOT NULL,
    title VARCHAR(255) NOT NULL,
    date DATE NOT NULL,
    time TIME NOT NULL,
    location TEXT NOT NULL,
    courts INT DEFAULT 1,
    max_players_per_court INT,
    total_slots INT NOT NULL,
    available_slots INT NOT NULL,
    price_vnd INT,
    price_usd DECIMAL(10,2),
    subscriber_early_access_hours INT,
    drop_in_cancellation_hours INT,
    subscriber_cancellation_hours INT,
    qr_code_url TEXT,
    cancelled BOOLEAN DEFAULT FALSE,
    cancelled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_sessions_date ON sessions(date);
CREATE INDEX idx_sessions_organizer ON sessions(organizer_id);
CREATE INDEX idx_sessions_cancelled ON sessions(cancelled);

-- Bookings table
CREATE TABLE bookings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) NOT NULL,
    session_id UUID REFERENCES sessions(id) NOT NULL,
    booking_code VARCHAR(20) UNIQUE NOT NULL,
    guest_count INT DEFAULT 0,
    tickets_used INT DEFAULT 0,
    discount_applied VARCHAR(20) DEFAULT 'none',
    price_paid_vnd INT NOT NULL,
    price_paid_usd DECIMAL(10,2),
    guest_price_paid_vnd INT DEFAULT 0,
    guest_price_paid_usd DECIMAL(10,2),
    payment_method VARCHAR(20) NOT NULL,
    payment_status VARCHAR(20) DEFAULT 'pending',
    verification_status VARCHAR(20),
    payment_screenshot_url TEXT,
    stripe_payment_id VARCHAR(255),
    payment_deadline TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_bookings_user ON bookings(user_id);
CREATE INDEX idx_bookings_session ON bookings(session_id);
CREATE INDEX idx_bookings_status ON bookings(payment_status);
CREATE INDEX idx_bookings_code ON bookings(booking_code);
CREATE INDEX idx_bookings_deadline ON bookings(payment_deadline) WHERE payment_status = 'pending';

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sessions_updated_at BEFORE UPDATE ON sessions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bookings_updated_at BEFORE UPDATE ON bookings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_config_updated_at BEFORE UPDATE ON config
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
