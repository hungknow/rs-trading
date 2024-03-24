#[cfg(any(target_arch = "wasm32"))]
pub trait HkConcurrent {}

#[cfg(any(target_arch = "wasm32"))]
impl<T: ?Sized> HkConcurrent for T {}

#[cfg(any(target_arch = "wasm32"))]
pub type HkBoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + 'a>>;

#[cfg(all(not(target_arch = "wasm32")))]
pub trait HkConcurrent: Send + Sync {}

#[cfg(all(not(target_arch = "wasm32")))]
impl<T: Send + Sync> HkConcurrent for T {}

#[cfg(all(not(target_arch = "wasm32")))]
pub type HkBoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
