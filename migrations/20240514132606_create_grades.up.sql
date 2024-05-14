-- Add up migration script here
CREATE TABLE grades (
    id SERIAL PRIMARY KEY,
    grade DECIMAL(5, 2) NOT NULL CHECK (
        grade >= 0
        AND grade <= 100
    ),
    grade_type_id SERIAL NOT NULL REFERENCES grade_types (id),
    user_id VARCHAR(32) NOT NULL REFERENCES users (id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);