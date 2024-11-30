use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

async fn async_hello() -> String {
    "Hello, async world!".to_string()
}

fn main() {
    let future = async_hello();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(future);
    println!("{}", result);
}
