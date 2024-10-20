-- Add migration script here
CREATE TABLE users (
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	registered_at timestamptz NOT NULL
);
