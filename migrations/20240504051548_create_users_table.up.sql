-- Add up migration script here
CREATE TABLE users (
    id VARCHAR(32) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(8) NOT NULL,
    password VARCHAR(60) NOT NULL
);

INSERT INTO
    users (id, name, role, password)
VALUES (
        'admin',
        'admin',
        'admin',
        '$2a$12$l9WF/lJhtrpRB//ymkNmR.IzTkRra8zDkMeDPguKMdjKTDNTq8dBG'
    );