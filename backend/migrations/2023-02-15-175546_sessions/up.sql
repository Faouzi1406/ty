-- Your SQL goes here
CREATE TABLE sessions (
  id  SERIAL PRIMARY KEY,
  sessions_key VARCHAR NOT NULL, 
  user_id INT NOT NULL, 
  CONSTRAINT fk_author FOREIGN KEY(user_id) REFERENCES users(id),
  date TIMESTAMP NOT NULL default now()
)
