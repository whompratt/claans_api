CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name CHARACTER varying NOT NULL,
  claan_id INTEGER NOT NULL REFERENCES claans(id),
  active boolean DEFAULT true NOT NULL
);
