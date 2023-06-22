-- Add migration script here
CREATE TABLE IF NOT EXISTS pairs (
    userid  SERIAL NOT NULL PRIMARY KEY,
    initial VARCHAR(1)  NOT NULL,
    next    VARCHAR(1)  NOT NULL,
    object  VARCHAR(32) NOT NULL,
    image   TEXT
);
