-- Your SQL goes here
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  comment VARCHAR(255) NOT NULL,
  user_id INT NOT NULL,
  video_id INT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (video_id) REFERENCES video(id)
)

