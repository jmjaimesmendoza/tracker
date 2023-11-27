-- Your SQL goes here
CREATE TABLE manuals (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    equipment_id INTEGER NOT NULL,
    FOREIGN KEY (equipment_id) REFERENCES equipments (id)
)