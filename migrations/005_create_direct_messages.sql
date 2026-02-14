-- +migrate Up
CREATE TABLE direct_messages (
    id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id    UUID        REFERENCES users(id),
    recipient_id UUID        REFERENCES users(id),
    content      TEXT        NOT NULL,
    is_read      BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_dm_sender    ON direct_messages(sender_id);
CREATE INDEX idx_dm_recipient ON direct_messages(recipient_id);

-- +migrate Down
DROP TABLE IF EXISTS direct_messages;
