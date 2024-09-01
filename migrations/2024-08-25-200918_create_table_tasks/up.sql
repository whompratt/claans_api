CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  description CHARACTER varying NOT NULL,
  tasktype tasktype NOT NULL,
  dice dice NOT NULL,
  ephemeral boolean NOT NULL default false,
  active boolean NOT NULL default false,
  last DATE
);
