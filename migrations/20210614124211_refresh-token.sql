CREATE TABLE IF NOT EXISTS refresh_token (
    tk_id SERIAL PRIMARY KEY,
    token VARCHAR,
    user_id INT,
    expiration TIMESTAMPTZ,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);
