#![feature(impl_trait_in_assoc_type)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use volo_redis::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::volo::redis::RedisServiceServer::new(S {
        map: Mutex::new(HashMap::<String, String>::new()),
    })
    .run(addr)
    .await
    .unwrap();
}
