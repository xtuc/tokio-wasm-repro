use std::future::Future;
use std::pin::Pin;
use std::task;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    let fut = TestFuture {};
    rt.block_on(fut);
}

pub struct TestFuture;

impl Future for TestFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        println!("paul");
        task::Poll::Pending
    }
}
