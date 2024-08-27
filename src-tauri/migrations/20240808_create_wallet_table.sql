CREATE TABLE IF NOT EXISTS wallets (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    seed TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);