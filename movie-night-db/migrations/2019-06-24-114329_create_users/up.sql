CREATE TABLE users
(
    id    SERIAL PRIMARY KEY,
    nick  TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
)