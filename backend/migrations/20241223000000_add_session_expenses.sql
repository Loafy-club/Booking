-- Add session_expenses table for tracking session costs and profit calculation

CREATE TABLE session_expenses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE NOT NULL,
    category VARCHAR(50) NOT NULL,
    description VARCHAR(255),
    cost_type VARCHAR(20) NOT NULL DEFAULT 'total',
    amount_vnd INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Ensure valid category values
    CONSTRAINT check_expense_category CHECK (category IN ('court_rental', 'equipment', 'instructor', 'custom')),

    -- Ensure valid cost_type values
    CONSTRAINT check_expense_cost_type CHECK (cost_type IN ('per_court', 'total')),

    -- Custom expenses must have a description
    CONSTRAINT check_custom_has_description CHECK (category != 'custom' OR description IS NOT NULL)
);

-- Index for efficient queries by session
CREATE INDEX idx_session_expenses_session_id ON session_expenses(session_id);

-- Index for category-based aggregations
CREATE INDEX idx_session_expenses_category ON session_expenses(category);

-- Trigger for updated_at
CREATE TRIGGER update_session_expenses_updated_at
    BEFORE UPDATE ON session_expenses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
