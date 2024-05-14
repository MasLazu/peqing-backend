-- Add up migration script here
CREATE TABLE users (
    id VARCHAR(32) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(8) NOT NULL,
    password VARCHAR(60) NOT NULL,
    qr_link TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO
    users (
        id,
        name,
        role,
        password,
        qr_link
    )
VALUES (
        'admin',
        'admin',
        'Admin',
        '$2a$12$l9WF/lJhtrpRB//ymkNmR.IzTkRra8zDkMeDPguKMdjKTDNTq8dBG',
        '-'
    );