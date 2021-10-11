CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password TEXT NOT NULL,
    salt BYTEA NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

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

CREATE TABLE IF NOT EXISTS sources(
    source_id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    website VARCHAR NOT NULL,
    feed_link VARCHAR NOT NULL,
    last_updated TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS user_subscriptions(
    sub_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL, 
    source_id INT NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(user_id)
            ON DELETE CASCADE,
    CONSTRAINT fk_source
        FOREIGN KEY(source_id)
            REFERENCES sources(source_id)
            ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS articles(
    article_id SERIAL PRIMARY KEY,
    source_id INT NOT NULL,
    item_link VARCHAR,
    title VARCHAR,
    description VARCHAR,
    author VARCHAR,
    pub_date TIMESTAMPTZ,
    content VARCHAR,
    guid VARCHAR,
    CONSTRAINT fk_source
        FOREIGN KEY(source_id)
            REFERENCES sources(source_id)
            ON DELETE CASCADE
);
