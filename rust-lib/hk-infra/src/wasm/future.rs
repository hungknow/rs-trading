pub trait HkConcurrent {}

impl<T: ?Sized> HkConcurrent for T {}

pub type HkBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub struct HkFutureResult<T, E> {
    pub fut: Pin<Box<dyn Future<Output = Result<T, e>>>>,
}

impl<T, E> HkFutureResult<T, E> {
    pub fn new<F>(f: F) -> Self
    where
        F: Future<Output = Result<T, E>> + 'static,
    {
        Self { fut: Box::pin(f) }
    }
}

impl<T, E> Future for HkFutureResult<T, E>
where
    E: Debug,
{
    type Output = Result<T, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        let result = ready!(this.fut.poll(cx));
        Poll::Ready(result)
    }
}
