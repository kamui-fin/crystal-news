CREATE TABLE IF NOT EXISTS refresh_token (
    tk_id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL,
    user_id INT NOT NULL,
    expiration TIMESTAMPTZ NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);
