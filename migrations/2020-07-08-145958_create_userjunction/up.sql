CREATE TABLE user_junctions(
    id INTEGER PRIMARY KEY NOT NULL,
    user_id VARCHAR NOT NULL,
    inventory_id INTEGER NOT NULL,
    location_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (inventory_id) REFERENCES inventories(id),
    FOREIGN KEY (location_id) REFERENCES locations(id)
)