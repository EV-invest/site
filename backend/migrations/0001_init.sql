-- Initial schema.
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS blogs (
	id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
	title      TEXT        NOT NULL,
	slug       TEXT        NOT NULL UNIQUE,
	body       TEXT        NOT NULL,
	published  BOOLEAN     NOT NULL DEFAULT FALSE,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
