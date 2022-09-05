-- Your SQL goes here

CREATE TABLE accounts (
	id SERIAL PRIMARY KEY, -- I'd loved a UUID here but I could not get diesel to accept it
	email VARCHAR NOT NULL UNIQUE,
	password VARCHAR NOT NULL,
	firstname VARCHAR NOT NULL,
	lastname VARCHAR NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	deleted_at timestamp DEFAULT NULL
)
