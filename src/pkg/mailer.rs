use std::fmt::Debug;

use super::email_templates::EmailTemplate;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Serialize;

#[derive(Debug)]
pub struct Mailer<T> {
    pub recipient: String,
    template: EmailTemplate,
    pub data: Option<T>,
}

impl<T: Serialize + Debug> Mailer<T> {
    pub fn new(recipient: &str, template: EmailTemplate, data: Option<T>) -> Self {
        match data {
            Some(data) => Self {
                recipient: recipient.to_string(),
                template,
                data: Some(data),
            },
            None => Self {
                recipient: recipient.to_string(),
                template,
                data: None,
            },
        }
    }

    pub async fn send_email(&self) {
        println!("sending email, {:?}", self);

        // Self::test_redis_connection().await.unwrap();
        // Self::test_smtp_connection().await;
        // let email = Message::builder()
        //     .from("NoBody <nobody@domain.tld>".parse().unwrap())
        //     .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        //     .to("Hei <hei@domain.tld>".parse().unwrap())
        //     .subject("Happy new year")
        //     .header(ContentType::TEXT_PLAIN)
        //     .body(String::from("Be happy!"))
        //     .unwrap();

        // let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

        // // Open a remote connection to gmail
        // let mailer = SmtpTransport::relay("smtp.gmail.com")
        //     .unwrap()
        //     .credentials(creds)
        //     .build();

        // // Send the email
        // match mailer.send(&email) {
        //     Ok(_) => println!("Email sent successfully!"),
        //     Err(e) => panic!("Could not send email: {e:?}"),
        // }
    }

    async fn test_smtp_connection() {
        println!("testing smtp connection")
    }

    async fn test_redis_connection() {
        // let client = RedisClient::default();
        // let _ = client.connect();
        // client.wait_for_connect().await?;

        // client.publish("test_channel", "hello world").await?;

        // println!("connected to redis");
        // Ok(())
    }
}
