-- Table: public.hiragana

-- DROP TABLE IF EXISTS public.hiragana;

CREATE TABLE IF NOT EXISTS public.hiragana
(
    id text COLLATE pg_catalog."default" NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT hiragana_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.hiragana
    OWNER to postgres;
