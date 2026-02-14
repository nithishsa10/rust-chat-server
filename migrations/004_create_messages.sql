-- +migrate Up
CREATE TABLE messages (
    id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id      UUID        REFERENCES rooms(id) ON DELETE CASCADE,
    user_id      UUID        REFERENCES users(id),
    content      TEXT        NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text',
    metadata     JSONB,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_messages_room_id    ON messages(room_id);
CREATE INDEX idx_messages_created_at ON messages(created_at DESC);

-- +migrate Down
DROP TABLE IF EXISTS messages;
