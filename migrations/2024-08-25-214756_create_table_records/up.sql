CREATE TABLE records (
    id SERIAL PRIMARY KEY,
    score INTEGER NOT NULL,
    "timestamp" DATE NOT NULL,
    task_id INTEGER NOT NULL REFERENCES tasks(id),
    user_id INTEGER NOT NULL REFERENCES "users"(id)
);