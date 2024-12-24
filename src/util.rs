#[cfg(not(target_arch = "wasm32"))]
use async_std::task;

use std::future::Future;

#[cfg(target_arch = "wasm32")]
pub fn spawn_thread<F: Future>(callback: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(callback);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_thread<F>(callback: F)
where
    F: Future<Output = ()> + 'static + Future + Send,
{
    task::spawn(callback);
}
