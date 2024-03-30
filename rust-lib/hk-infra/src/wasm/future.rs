pub trait HkConcurrent: Send + Sync {}

pub struct HkFutureResult<T, E> {
    pub fut: Pin<Box<dyn Future<Output = Result<T, e>> + 'static>>,
}

impl<T, E> HkFutureResult<T, E> {
    pub fn new<F>(f: F) -> Self
    where
        F: Future<Output = Result<T, E>> + 'static,
    {
        Self { fut: Box::pin(f) }
    }
}

impl<T, E> Future for HkFutureResult<T, E> {
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut.as_mut().poll(cx)
    }
}
