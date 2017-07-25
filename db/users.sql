DROP TABLE IF EXISTS users;

CREATE TABLE users (
		user_id BYTES PRIMARY KEY DEFAULT uuid_v4(),
		username STRING UNIQUE NOT NULL,
		salt BYTES NOT NULL,
		password BYTES NOT NULL,
		active BOOL DEFAULT true,
		created_on TIMESTAMPTZ NOT NULL
);
