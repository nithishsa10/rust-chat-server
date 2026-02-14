-- +migrate Up
CREATE TABLE room_members (
    room_id   UUID REFERENCES rooms(id) ON DELETE CASCADE,
    user_id   UUID REFERENCES users(id) ON DELETE CASCADE,
    role      VARCHAR(20) NOT NULL DEFAULT 'member',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (room_id, user_id)
);
CREATE INDEX idx_room_members_user_id ON room_members(user_id);

-- +migrate Down
DROP TABLE IF EXISTS room_members;
