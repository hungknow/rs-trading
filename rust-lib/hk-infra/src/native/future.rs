use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project::pin_project;

pub trait HkConcurrent: Send + Sync {}

impl<T: Send + Sync> HkConcurrent for T {}

pub type HkBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[pin_project]
pub struct HkFutureResult<T, E> {
    #[pin]
    pub fut: Pin<Box<dyn Future<Output = Result<T, E>> + Sync + Send>>,
}

impl<T, E> HkFutureResult<T, E> {
    pub fn new<F>(f: F) -> Self
    where
        F: Future<Output = Result<T, E>> + Sync + Send + 'static,
    {
        Self { fut: Box::pin(f) }
    }
}

impl<T, E> Future for HkFutureResult<T, E>
where
    T: Send + Sync,
    E: Debug,
{
    type Output = Result<T, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        let result = ready!(this.fut.poll(cx));
        Poll::Ready(result)
    }
}
