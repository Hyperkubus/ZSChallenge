-- Your SQL goes here

CREATE TABLE cardetails (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	account_id UUID,
	owner VARCHAR NOT NULL,
	plate VARCHAR(15) NOT NULL,
	registration VARCHAR NOT NULL, -- LINK TO IMAGE ON S3
	CONSTRAINT fk_accounts
		FOREIGN KEY(account_id)
			REFERENCES accounts(id)
)
