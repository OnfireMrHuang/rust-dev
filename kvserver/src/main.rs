use anyhow::Result;
use async_prost::AsyncProstStream;
use dashmap::DashMap;
use futures::prelude::*;
use kv::{
    command_request::RequestData, CommandRequest, CommandResponse,Hset,KvError,Kvpair,Value,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 定义监听地址
    let addr = "127.0.0.1:9527";


}
