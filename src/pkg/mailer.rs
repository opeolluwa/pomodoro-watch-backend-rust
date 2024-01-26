/// the module contains the mailer logic to send emails to users
/// the mailer is a redis pubsub client that subscribes  to a channel form where it receives publised elesewhere un the application
use fred::prelude::*;
use serde::Serialize;

// / email template names
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

pub struct Mailer<T> {
    pub recipient: String,
    template: EmailTemplate,
    pub data: Option<T>,
}

impl<T: Serialize> Mailer<T> {
    pub fn new(recipient: &str, template: EmailTemplate, data: Option<T>) -> Self {
        match data {
            Some(data) => {
                Self {
                    recipient: recipient.to_string(),
                    template,
                    data: Some(data),
                }
            }
            None => {
                Self {
                    recipient: recipient.to_string(),
                    template,
                    data: None,
                }
            }
        }
    }

    pub async fn send_email(&self) {
        // Self::test_redis_connection().await.unwrap();
        // Self::test_smtp_connection().await;

    }

    async fn test_smtp_connection() {
        println!("testing smtp connection")
    }

    async fn test_redis_connection()  {
        // let client = RedisClient::default();
        // let _ = client.connect();
        // client.wait_for_connect().await?;

        // client.publish("test_channel", "hello world").await?;

        // println!("connected to redis");
        // Ok(())
    }
}
