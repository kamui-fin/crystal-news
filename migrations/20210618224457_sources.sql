CREATE TABLE IF NOT EXISTS sources(
    source_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL, 
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    website VARCHAR NOT NULL,
    feed_link VARCHAR NOT NULL,
    last_updated TIMESTAMPTZ,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(user_id)
            ON DELETE CASCADE
);
