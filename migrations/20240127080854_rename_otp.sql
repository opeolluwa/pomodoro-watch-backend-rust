-- Add migration script here
ALTER TABLE one_time_passwords RENAME COLUMN id TO otp_id;
