use redis_async_pool::{RedisPool, RedisConnectionManager};
use redis_cluster_async::redis::AsyncCommands;
use crate::orm_redis::conv_data::ConvData;

#[macro_use]
extern crate orm_redis;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> common_uu::IResult {
    #[derive(RedisZrange)]
    struct Demo3 {
        id: i64,
        name: String,
    }

    let hosts: Vec<String> = "127.0.0.1:3302".split(",").map(|v| v.to_string()).collect();
    let mut client = redis_cluster_async::Client::open(hosts)?;
    let redis_pool = RedisPool::new(RedisConnectionManager::new(client, true, None), 4);
    let mut conn = redis_pool.get().await?;
    let v: Demo3 = conn.hget("", ("id", "name")).await?;
    Ok(())
}

#[tokio::test]
async fn main_vec_keys() -> common_uu::IResult {
    #[derive(RedisHget)]
    struct Demo {
        id: i64,
        name: String,
    }

    let hosts: Vec<String> = "127.0.0.1:3302".split(",").map(|v| v.to_string()).collect();
    let mut client = redis_cluster_async::Client::open(hosts)?;
    let redis_pool = RedisPool::new(RedisConnectionManager::new(client, true, None), 4);
    let mut conn = redis_pool.get().await?;
    let v: Demo = conn.hget("", vec!["id", "name"]).await?;
    Ok(())
}

#[tokio::test]
async fn test_keys() -> common_uu::IResult {
    #[derive(RedisFromValue)]
    struct Demo {
        id: i64,
        name: String,
    }

    let hosts: Vec<String> = "127.0.0.1:3302".split(",").map(|v| v.to_string()).collect();
    let mut client = redis_cluster_async::Client::open(hosts)?;
    let redis_pool = RedisPool::new(RedisConnectionManager::new(client, true, None), 4);
    let mut conn = redis_pool.get().await?;
    let v: Demo = conn.hget("", Demo::keys()).await?;
    Ok(())
}
