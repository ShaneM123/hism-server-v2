CREATE TABLE profiles (
                       profile_id INTEGER PRIMARY KEY NOT NULL,
                       id VARCHAR NOT NULL,
                       bio VARCHAR NOT NULL,
                       age INTEGER NOT NULL,
                       community VARCHAR NOT NULL,
                       FOREIGN KEY (id) REFERENCES users(id)

)