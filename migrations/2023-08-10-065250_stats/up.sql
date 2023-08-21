-- Your SQL goes here
CREATE TABLE stats (
    id UUID NOT NULL PRIMARY KEY,
    ip_address VARCHAR(255) NOT NULL UNIQUE,
    ping_count INT NOT NULL,
    join_count INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE players(
    uuid UUID NOT NULL PRIMARY KEY,
    server_uuid UUID NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_player
        FOREIGN KEY (server_uuid)
        REFERENCES stats (id)
        ON DELETE CASCADE
);
