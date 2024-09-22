CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name CHARACTER varying NOT NULL,
  claan_id INTEGER NOT NULL REFERENCES claans(id),
  email VARCHAR(120) UNIQUE NOT NULL,
  password_hash BYTEA NOT NULL,
  active boolean DEFAULT true NOT NULL,
  current_auth_token VARCHAR(23),
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  last_action TIMESTAMP
);
