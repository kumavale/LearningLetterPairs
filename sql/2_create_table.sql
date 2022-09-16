-- Table: public.list

-- DROP TABLE IF EXISTS public.list;

\c letterpairs

CREATE TABLE IF NOT EXISTS public.list
(
    initial text COLLATE pg_catalog."default" NOT NULL,
    next text COLLATE pg_catalog."default" NOT NULL,
    objects text[] COLLATE pg_catalog."default" NOT NULL,
    image text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT list_pkey PRIMARY KEY (initial, next)
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.list
    OWNER to postgres;