-- Add up migration script here
CREATE TABLE grade_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    subject_id SERIAL NOT NULL REFERENCES subjects (id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);