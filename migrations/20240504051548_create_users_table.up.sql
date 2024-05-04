-- Add up migration script here
CREATE TABLE users (
    id VARCHAR(32) PRIMARY KEY, name VARCHAR(255) NOT NULL, role VARCHAR(8) NOT NULL, password VARCHAR(255) NOT NULL
);