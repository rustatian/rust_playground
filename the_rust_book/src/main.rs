#![allow(dead_code)]
use std::future::Future;

fn main() {

}

fn bar() -> impl Future<Output = u8> {
    async {
        let x:u8 = foo().await;
        x + 5
    }
}

async fn foo() -> u8 {
    5
}