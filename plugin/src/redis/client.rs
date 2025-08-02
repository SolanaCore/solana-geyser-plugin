use redis::{Client, Commands, RedisResult};

/// Redis connection manager
#[derive(Debug)]
pub struct RedisManager {
    client: Client,
}

impl RedisManager {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub fn publish(&self, channel: &str, message: &str) -> RedisResult<()> {
        let mut connection = self.client.get_connection()?;
        connection.publish::<_, _, ()>(channel, message)?; // Explicit () return type
        Ok(())
    }
}
