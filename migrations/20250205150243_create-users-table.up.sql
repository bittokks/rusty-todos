-- Add up migration script here
CREATE TABLE users (
                 id  UUID PRIMARY KEY NOT NULL DEFAULT (gen_random_uuid()),
                                     username  VARCHAR(48) UNIQUE NOT NULL,
                                       email  VARCHAR(100) UNIQUE NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT (now())
);
