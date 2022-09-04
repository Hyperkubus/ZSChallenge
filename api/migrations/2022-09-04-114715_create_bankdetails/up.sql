-- Your SQL goes here

CREATE TABLE bankdetails (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	account_id UUID,
	holder VARCHAR NOT NULL,
	iban VARCHAR NOT NULL,
	bic VARCHAR NOT NULL,
	CONSTRAINT fk_accounts
		FOREIGN KEY(account_id)
			REFERENCES accounts(id)
)
