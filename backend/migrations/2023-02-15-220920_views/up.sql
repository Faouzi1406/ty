-- Your SQL goes here
CREATE TABLE views (
    id SERIAL PRIMARY KEY,
    video_id INTEGER NOT NULL, 
    user_id INTEGER NOT NULL,
    FOREIGN KEY (video_id) REFERENCES video(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
