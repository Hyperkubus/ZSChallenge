-- Your SQL goes here

CREATE TABLE bankdetails (
	id SERIAL PRIMARY KEY,
	account_id SERIAL NOT NULL,
	holder VARCHAR NOT NULL,
	iban VARCHAR NOT NULL,
	bic VARCHAR NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NOT NULL,
	deleted_at timestamp DEFAULT NULL,
	CONSTRAINT fk_accounts
		FOREIGN KEY(account_id)
		REFERENCES accounts(id)
		ON DELETE CASCADE
)
