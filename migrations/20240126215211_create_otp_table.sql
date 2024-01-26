-- Add migration script here
-- the one time password contains otp, a unix timestamp and the userId which is a foreign key yo user information.id 

CREATE TABLE IF NOT EXISTS one_time_passwords (
    id UUID PRIMARY KEY,
    otp VARCHAR(6) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_information(id)
);