CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  claan_id INTEGER NOT NULL REFERENCES claans(id),
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  name CHARACTER varying NOT NULL,
  email VARCHAR(120) UNIQUE NOT NULL,
  password_hash BYTEA NOT NULL,
  current_auth_token VARCHAR(23),
  last_action TIMESTAMP,
  active boolean DEFAULT true NOT NULL
);
