
use crate::error::{Error, Result};
pub async fn redis_connection ()->Result<redis::aio::MultiplexedConnection> {
let client = redis::Client::open("redis://127.0.0.1/").map_err(|e| Error::RedisError(e))?;
let mut con = client.get_multiplexed_async_connection().await?;

Ok(con)

}


