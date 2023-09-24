-- Your SQL goes here
CREATE TABLE equipments (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  km INTEGER NOT NULL
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