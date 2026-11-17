use crate::prelude::*;

#[allow(missing_docs)]
pub type SignalHandler = Box<dyn Fn() + Send + Sync>;

#[cfg(has_tls)]
pub fn lazy_per_thread_init() {}
