DROP TABLE IF EXISTS activities;

CREATE TABLE activities (
		id SERIAL PRIMARY KEY,
		user_id BYTES NOT NULL REFERENCES users (user_id),
		filename STRING NOT NULL,
		type STRING,
		name STRING
);
