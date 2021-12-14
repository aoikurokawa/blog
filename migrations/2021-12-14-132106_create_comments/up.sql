CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    post_id INTEGER NOT NULL REFERENCES posts (id),
    body TEXT NOT NULL
)