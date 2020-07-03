CREATE TABLE locations (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id VARCHAR NOT NULL,
    geo_location INTEGER NOT NULL,
    home_location INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
)