-- +migrate Up
CREATE TABLE rooms (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(100) NOT NULL,
    description TEXT,
    is_private  BOOLEAN     NOT NULL DEFAULT FALSE,
    created_by  UUID        REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_rooms_name ON rooms(name);

-- +migrate Down
DROP TABLE IF EXISTS rooms;
