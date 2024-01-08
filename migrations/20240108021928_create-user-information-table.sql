-- Add migration script here
CREATE TABLE user_information(
    id uuid PRIMARY KEY NOT NULL,
    full_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    occupation VARCHAR -- nullable
);