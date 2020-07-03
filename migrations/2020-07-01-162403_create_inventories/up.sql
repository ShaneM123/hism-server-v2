CREATE TABLE inventories (
                       id INTEGER PRIMARY KEY NOT NULL,
                       user_id VARCHAR NOT NULL,
                       total_items INTEGER NOT NULL,
                       FOREIGN KEY (user_id) REFERENCES users(id)

)