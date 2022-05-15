-- Your SQL goes here
CREATE TABLE products (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	image TEXT NOT NULL,
	description TEXT NOT NULL,
	price INTEGER NOT NULL
);

INSERT INTO products (name, image, description, price)
VALUES ("Auta", "https://farm8.staticflickr.com/7457/13104485265_ddd94e847c_o.jpg", "<p>Opis produktu</p>", 2550),
	   ("Auta 2", "https://vignette.wikia.nocookie.net/pixar/images/4/45/E7306E15-54FB-42F1-A576-9C844CBB468F.jpeg/revision/latest?cb=20181029000828", "<p>Opis produktu</p>", 3099);
