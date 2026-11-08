use std::pin::Pin;

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
