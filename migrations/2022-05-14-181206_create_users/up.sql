-- Your SQL goes here
CREATE TABLE users (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	email TEXT NOT NULL,
	password CHAR(64) NOT NULL,
	privilege INTEGER NOT NULL DEFAULT 1
);

INSERT INTO users (name, email, password, privilege)
VALUES ("admin", "admin@admin.admin", "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918", 1000);
