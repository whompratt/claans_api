CREATE TABLE seasons (
  id SERIAL PRIMARY KEY,
  name CHARACTER varying NOT NULL,
  start_date DATE NOT NULL
);

INSERT INTO seasons (name, start_date) VALUES ('Default', DATE '1970-01-01');