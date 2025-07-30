-- Initial migration for Cashu mint
CREATE TABLE IF NOT EXISTS mint_keysets (
    id TEXT PRIMARY KEY,
    currency TEXT NOT NULL,
    keyset_data TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS mint_quotes (
    id TEXT PRIMARY KEY,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    state TEXT NOT NULL,
    request TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER
);

CREATE TABLE IF NOT EXISTS proofs (
    id TEXT PRIMARY KEY,
    amount INTEGER NOT NULL,
    secret TEXT NOT NULL,
    c TEXT NOT NULL,
    keyset_id TEXT NOT NULL,
    spent BOOLEAN DEFAULT FALSE,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_proofs_secret ON proofs(secret);
CREATE INDEX IF NOT EXISTS idx_proofs_keyset ON proofs(keyset_id);
CREATE INDEX IF NOT EXISTS idx_quotes_state ON mint_quotes(state);