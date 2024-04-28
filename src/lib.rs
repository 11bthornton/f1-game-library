#![feature(coroutines)]

pub mod event_loop;
pub mod telemetry_data;

// #[cfg(all(feature = "sync", not(nightly)))]
// compile_error!("The 'sync' feature requires nightly Rust.");

// #[cfg(all(feature = "sync", nightly))]
