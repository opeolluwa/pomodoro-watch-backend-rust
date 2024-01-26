-- Add migration script here
ALTER TABLE user_information ADD COLUMN is_verified BOOLEAN NOT NULL DEFAULT false;