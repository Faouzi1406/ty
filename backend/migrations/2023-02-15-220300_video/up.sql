-- Your SQL goes here
CREATE TABLE video (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    url VARCHAR NOT NULL,
    thumb_mail_url VARCHAR NOT NULL DEFAULT  'default.jpeg',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id INTEGER NOT NULL REFERENCES users(id)
);
