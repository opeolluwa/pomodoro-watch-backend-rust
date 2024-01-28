pub(crate) mod auth {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Signup {
        pub full_name: String,
        pub password: String,
        pub email: String,
        pub occupation: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VerifyEmail {
        pub otp: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NewVerificationToken {
        pub email: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Login {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ForgottenPassword {
        pub email: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Otp {
        pub otp: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ResetPassword {
        pub otp: String,
        pub password: String,
        pub confirm_password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConfirmPasswordReset {
        pub otp: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NewPassword {
        pub new_password: String,
        pub confirm_password: String,
    }
}

mod user {}

mod pomodoro {}
