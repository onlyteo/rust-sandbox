CREATE TABLE users
(
    id               UUID PRIMARY KEY,
    -- Cached payload from the external system (NULL until first successful sync)
    name             TEXT,
    email            TEXT,
    data_updated_at  TIMESTAMPTZ,                       -- last time the payload actually changed
    -- Sync metadata
    status           VARCHAR(12) NOT NULL DEFAULT 'IDLE' CHECK (status IN ('IDLE', 'PROCESSING')),
    claimed_by       UUID,                              -- instance id holding the lease
    lease_expires_at TIMESTAMPTZ,                       -- lease is void after this point
    next_sync_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_synced_at   TIMESTAMPTZ,
    failure_count    INTEGER     NOT NULL DEFAULT 0,
    last_error       TEXT
);

CREATE INDEX users_next_sync_at_idx ON users (next_sync_at);
