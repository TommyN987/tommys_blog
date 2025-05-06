-- Add up migration script here

CREATE TABLE posts (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL UNIQUE,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

