-- Add migration script here
CREATE TABLE IF NOT EXISTS pairs (
    id      INTEGER     NOT NULL,
    initial VARCHAR(1)  NOT NULL,
    next    VARCHAR(1)  NOT NULL,
    object  VARCHAR(32) NOT NULL,
    image   TEXT,
    CONSTRAINT PK_pairs PRIMARY KEY (id,initial,next)
);
