CREATE TABLE items (
                       id INTEGER PRIMARY KEY NOT NULL,
                       inventory_id INTEGER NOT NULL,
                       dimensions INTEGER NOT NULL,
                       weight INTEGER NOT NULL,
                       value INTEGER NOT NULL,
                       description VARCHAR NOT NULL,
                       FOREIGN KEY (inventory_id) REFERENCES inventories(id)

)