-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE CHECK (char_length(username) > 6),
  password VARCHAR NOT NULL CHECK (char_length(password) > 6),
  email VARCHAR NOT NULL UNIQUE CHECK (char_length(email) > 6),
  profile_pic VARCHAR NOT NULL DEFAULT 'https://api.dicebear.com/5.x/big-smile/svg'
)
