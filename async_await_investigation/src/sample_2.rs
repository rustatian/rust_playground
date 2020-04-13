use futures::Future;
async fn foo() -> u32 {
    0
}

fn foo2() -> impl Future<Output = u32> {
    futures::future::ready(0)
}