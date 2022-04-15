// use anyhow::Result;
// use async_prost::AsyncProstStream;
// use dashmap::DashMap;
// use futures::prelude::*;
// use kv::{
//     command_request::RequestData, CommandRequest, CommandResponse,Hset,KvError,Kvpair,Value,
// };
// use std::sync::Arc;
// use tokio::net::TcpListener;
// use tracing::info;

// #[tokio::main]
// async fn main() -> Result<()> {
//     // 初始化日志
//     tracing_subscriber::fmt::init();

//     // 定义监听地址
//     let addr = "127.0.0.1:9527";
//     let listener = TcpListener::bind(addr).await?;
//     info!("Start listener on {}",addr);

//     // 使用DashMap创建放在内存中的kv store
//     let table: Arc<DashMap<string,Value>> = Arc::new(DashMap::new());

//     loop {
//         // 得到一个客户端请求
//         let (stream,addr) = listener.accept().await?;
//         info!("Client {:?} connected",addr);

//         // 复制db,让它在tokio任务中可以使用
//         let db = table.clone();

//         // 创建一个tokio任务处理这个客户端
//         tokio::spawn(async move {
//             // 使用asyncProstStream来处理TCP frame
//             // Frame: 两字节frame长度，后面是protobuf二进制
//             let mut stream = AsyncProstStream::<_,CommandRequest, CommandResponse,_>::from(stream).for_async();

//             // 从stream里取下一个消息(拿出来后已经自动decode了)
//             while let Some(Ok(msg)) = stream.next().await {
//                 info!("Got a new command: {:?}",msg);
//                 let resp: CommandResponse = match msg.request_data {
//                     // 为演示我们就处理HSET
//                     Some(RequestData::Hset(cmd)) => hset(cmd, &db),
//                     // 其他暂不处理
//                     _ => unimplemented!(),
//                 };

//                 info!("Got response: {:?}", resp);
//                 // 把CommandResponse发送给客户端
//                 stream.send(resp).await.unwrap();
//             }
//         });
//     }

// }

// fn hset(cmd: Hset, db: &DashMap<String,Value>) -> CommandResponse {
//     match cmd.pair {
//         Some(Kvpair {
//             key,
//             value: Some(v),
//         }) => {
//             // 往db里写入
//             let old = db.insert(key, v).unwrap_or_default();
//             // 把value转换为CommandResponse
//             old.into();
//         }
//         v => KvError::InvalidCommand(format!("hset: {:?}",v)).into(),
//     }
// }
