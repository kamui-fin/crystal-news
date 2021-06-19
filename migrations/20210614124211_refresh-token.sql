CREATE TABLE IF NOT EXISTS refresh_token (
    token_id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL,
    user_id INT NOT NULL,
    expiration TIMESTAMPTZ NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(user_id)
            ON DELETE CASCADE
);
