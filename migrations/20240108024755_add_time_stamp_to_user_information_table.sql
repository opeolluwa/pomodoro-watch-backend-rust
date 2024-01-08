-- Add migration script here
ALTER TABLE user_information ADD COLUMN created_at TIMESTAMP DEFAULT  NOW();
ALTER TABLE  user_information ADD COLUMN  updated_at TIMESTAMP DEFAULT NOW();