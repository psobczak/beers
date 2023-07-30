-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS beers(
    id uuid PRIMARY KEY NOT NULL,
    name VARCHAR(100) UNIQUE NOT NULL,
    alcohol_content FLOAT NOT NULL,
    producent VARCHAR(100) NOT NULL
);