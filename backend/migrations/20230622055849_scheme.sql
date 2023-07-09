-- Add migration script here
CREATE TABLE IF NOT EXISTS pairs (
    id      INTEGER     NOT NULL,
    initial VARCHAR(1)  NOT NULL,
    next    VARCHAR(1)  NOT NULL,
    object  VARCHAR(32) NOT NULL,
    image   TEXT,
    CONSTRAINT PK_pairs PRIMARY KEY (id,initial,next)
);
CREATE TABLE IF NOT EXISTS users (
    id            SERIAL       NOT NULL,
    username      VARCHAR(32)  NOT NULL,
    password_hash TEXT         NOT NULL,
    CONSTRAINT PK_users PRIMARY KEY (username)
);
