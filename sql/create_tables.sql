
.open users.db
CREATE TABLE users (
   id		 INTEGER PRIMARY KEY AUTOINCREMENT,
   name          TEXT UNIQUE NOT NULL,
   password      TEXT NOT NULL,
   created       TEXT NOT NULL
);