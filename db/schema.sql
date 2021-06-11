CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    salt BYTEA NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);
