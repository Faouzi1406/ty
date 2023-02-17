-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE CHECK (char_length(username) > 6),
  password VARCHAR NOT NULL CHECK (char_length(password) > 6),
  email VARCHAR NOT NULL UNIQUE CHECK (char_length(email) > 6)
)
