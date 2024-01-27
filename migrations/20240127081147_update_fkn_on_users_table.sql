-- Add migration script here
ALTER TABLE user_information DROP CONSTRAINT user_information_otp_id_fkey;

ALTER TABLE user_information ADD CONSTRAINT user_information_otp_id_fkey FOREIGN KEY (otp_id) REFERENCES one_time_passwords(otp_id);