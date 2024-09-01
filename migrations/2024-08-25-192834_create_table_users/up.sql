CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name CHARACTER varying NOT NULL,
  claan claan NOT NULL,
  active boolean DEFAULT true NOT NULL
);
