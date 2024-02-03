-- Add migration script here
CREATE TABLE real_distributions(
    name TEXT PRIMARY KEY,
    data JSONB NOT NULL
);
