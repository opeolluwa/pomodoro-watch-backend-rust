// user letter to send html emails
use fred::prelude::*;
pub struct Mailer {}

impl Mailer {
    pub async fn test_connection() {}

    pub async fn send_email() -> Result<(), RedisError> {
        let publisher_client = RedisClient::default();
        let subscriber_client = publisher_client.clone_new();
        todo!("implement this")
    }
}
