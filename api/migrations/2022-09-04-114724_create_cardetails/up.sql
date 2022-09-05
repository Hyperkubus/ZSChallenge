-- Your SQL goes here

CREATE TABLE cardetails (
	id SERIAL PRIMARY KEY,
	account_id SERIAL NOT NULL,
	owner VARCHAR NOT NULL,
	plate VARCHAR(15) NOT NULL,
	registration VARCHAR NOT NULL, -- LINK TO IMAGE ON S3
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NOT NULL,
	deleted_at timestamp DEFAULT NULL,
	CONSTRAINT fk_accounts
		FOREIGN KEY(account_id)
		REFERENCES accounts(id)
		ON DELETE CASCADE
)
