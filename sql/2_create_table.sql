\c letterpairs

-- list
CREATE TABLE IF NOT EXISTS public.list
(
    initial text   NOT NULL,
    next    text   NOT NULL,
    objects text[] NOT NULL,
    image   text   NOT NULL,
    CONSTRAINT list_pkey PRIMARY KEY (initial, next)
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.list OWNER to postgres;

-- users
CREATE TABLE IF NOT EXISTS public.users
(
    id       serial NOT NULL PRIMARY KEY,
    username text   NOT NULL,
    password text   NOT NULL,
    UNIQUE(username)
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.users OWNER to postgres;
