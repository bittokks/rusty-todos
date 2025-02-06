-- Add up migration script here
CREATE TABLE users (
  id UUID PRIMARY KEY NOT NULL DEFAULT (gen_random_uuid ()),
  username VARCHAR(48) NOT NULL UNIQUE,
  email VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(100) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT (now())
);
