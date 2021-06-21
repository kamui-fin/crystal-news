CREATE TABLE IF NOT EXISTS articles(
    article_id SERIAL PRIMARY KEY,
    source_id INT NOT NULL,
    item_link VARCHAR,
    title VARCHAR,
    description VARCHAR,
    author VARCHAR,
    content VARCHAR,
    CONSTRAINT fk_source
        FOREIGN KEY(source_id)
            REFERENCES sources(source_id)
            ON DELETE CASCADE
)
