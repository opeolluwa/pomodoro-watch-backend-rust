// / email template names
#[derive(Debug)]
pub enum EmailTemplate {
    Welcome,
    VerifyEmail,
    ResetPassword,
    NewPassword,
    NewVerificationToken,
}

impl EmailTemplate {
    /// get the template name as a string
    pub fn as_str(&self) -> &str {
        match self {
            Self::Welcome => "welcome",
            Self::VerifyEmail => "verify_email",
            Self::ResetPassword => "reset_password",
            Self::NewPassword => "new_password",
            Self::NewVerificationToken => "new_verification_token",
        }
    }
}

impl From<EmailTemplate> for String {
    fn from(template: EmailTemplate) -> Self {
        template.as_str().to_string()
    }
}

impl Default for EmailTemplate {
    fn default() -> Self {
        Self::Welcome
    }
}
