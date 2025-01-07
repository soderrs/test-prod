CREATE TABLE users (
    login TEXT NOT NULL,
    email TEXT NOT NULL,
    country_code TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_public BOOLEAN NOT NULL,
    phone TEXT,
    image TEXT,
    friends JSON
);
