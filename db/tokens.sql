DROP TABLE IF EXISTS user_tokens;

CREATE TABLE user_tokens (
		id SERIAL PRIMARY KEY,
		user_id BYTES UNIQUE NOT NULL REFERENCES users (user_id),
		token BYTES UNIQUE NOT NULL
);
