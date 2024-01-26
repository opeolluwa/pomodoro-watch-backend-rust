-- Add migration script here

ALTER TABLE one_time_passwords  DROP CONSTRAINT one_time_passwords_user_id_fkey;


ALTER TABLE one_time_passwords DROP COLUMN user_id;

