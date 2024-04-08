use tokio::runtime::Runtime;

pub struct HkDispatcherRuntime {
    inner: Runtime,
    #[cfg(any(target_arch = "wasm32", feature = "local_set"))]
    local: tokio::task::LocalSet,
}

