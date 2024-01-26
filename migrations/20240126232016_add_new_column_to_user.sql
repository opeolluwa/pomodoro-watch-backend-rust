-- Add migration script here
ALTER TABLE user_information ADD COLUMN otp_id UUID;

ALTER TABLE user_information ADD CONSTRAINT user_information_otp_id_fkey FOREIGN KEY (otp_id) REFERENCES one_time_passwords(id);
