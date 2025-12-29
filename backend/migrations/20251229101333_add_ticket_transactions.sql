-- Ticket transactions audit table
-- Tracks all ticket operations: grants, usage, restorations, bonuses, expirations, revocations
CREATE TABLE ticket_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) NOT NULL,
    subscription_id UUID REFERENCES subscriptions(id),
    booking_id UUID REFERENCES bookings(id),
    transaction_type VARCHAR(30) NOT NULL,
    -- Types: subscription_grant, used, restored, bonus_referral,
    --        bonus_birthday, bonus_manual, expired, revoked
    amount INT NOT NULL,  -- positive for credits, negative for debits
    balance_after INT NOT NULL,  -- snapshot of balance after transaction
    notes TEXT,
    admin_id UUID REFERENCES users(id),  -- for manual grants/revokes
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_ticket_tx_user ON ticket_transactions(user_id);
CREATE INDEX idx_ticket_tx_subscription ON ticket_transactions(subscription_id);
CREATE INDEX idx_ticket_tx_booking ON ticket_transactions(booking_id);
CREATE INDEX idx_ticket_tx_type ON ticket_transactions(transaction_type);
CREATE INDEX idx_ticket_tx_created ON ticket_transactions(created_at DESC);

-- Bonus tickets tracking table
-- Records each bonus ticket award with type and metadata
CREATE TABLE bonus_tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) NOT NULL,
    bonus_type VARCHAR(20) NOT NULL,  -- 'referral', 'birthday', 'manual'
    tickets INT NOT NULL DEFAULT 1,
    note TEXT,
    referrer_id UUID REFERENCES users(id),  -- For referral bonuses
    granted_by UUID REFERENCES users(id),   -- Admin who granted (for manual)
    year INT,  -- For birthday: track year to prevent duplicates
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_bonus_tickets_user ON bonus_tickets(user_id);
CREATE INDEX idx_bonus_tickets_type ON bonus_tickets(bonus_type);

-- Prevent duplicate birthday bonuses per year
CREATE UNIQUE INDEX idx_bonus_birthday_year ON bonus_tickets(user_id, year)
    WHERE bonus_type = 'birthday';
