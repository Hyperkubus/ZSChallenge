-- Your SQL goes here

CREATE TABLE accounts (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	email VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	firstname VARCHAR NOT NULL,
	lastname VARCHAR NOT NULL,
	created_at timestamp NOT NULL,
	updated_at timestamp NOT NULL,
	deleted_at timestamp
)
