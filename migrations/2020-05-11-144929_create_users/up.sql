CREATE TABLE users (
                       id VARCHAR PRIMARY KEY NOT NULL,
                       username VARCHAR NOT NULL,
                       password VARCHAR NOT NULL,
                       email VARCHAR NOT NULL,
                       profileid INTEGER FOREIGN KEY REFERENCES profiles )