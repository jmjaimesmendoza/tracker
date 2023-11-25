-- Your SQL goes here
CREATE TABLE brands (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE models (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  brand_id INTEGER NOT NULL,
  name TEXT NOT NULL,

  FOREIGN KEY (brand_id) REFERENCES brands (id)
);

INSERT INTO brands (name) VALUES ("John Deere"), 
("New Holland"),
("Same"),
("Massey Ferguson"),
("International"),
("Caterpillar"),
("Komatsu"),
("Stihl"),
("Shindaiwa"),
("Mardal"),
("Toyota"),
("Hiunday"),
("Chevrolet"),
("Ford"),
("Kawasaki");

CREATE TABLE equipments (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  km INTEGER NOT NULL,
  model_id INTEGER NOT NULL,
  nserial TEXT NOT NULL,
  notes TEXT NOT NULL,
  file_path TEXT NOT NULL,

  FOREIGN KEY (model_id) REFERENCES models (id)
);

CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  username TEXT NOT NULL,
  hash TEXT NOT NULL
);

CREATE TABLE persons (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  equipment_id INTEGER NOT NULL,
  person_id INTEGER NOT NULL,
  job TEXT NOT NULL CHECK( job IN ('Preventivo', 'Reparacion leve', 'Reparacion mayor', 'Cambio de aceite y filtros') ),
  km INTEGER NOT NULL,
  description TEXT NOT NULL,

  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY (equipment_id) REFERENCES equipments (id),
  FOREIGN KEY (person_id) REFERENCES persons (id)
);

CREATE TABLE revisions (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  equipment_id INTEGER NOT NULL,
  tipo TEXT NOT NULL CHECK ( tipo IN ('D','K')),
  target TEXT NOT NULL,

  FOREIGN KEY (equipment_id) REFERENCES equipments (id)
)