-- Your SQL goes here
CREATE TABLE stats (
    id SERIAL,
    ip_address VARCHAR(255) NOT NULL,
    ping_count INT NOT NULL,
    join_count INT NOT NULL,
    PRIMARY KEY (id)
);
