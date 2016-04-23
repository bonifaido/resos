#![deny(unused_mut)]
#[macro_use]
extern crate hyper;
extern crate protobuf;

// We want to reexport the messages as part of this crate.
// Otherwise one cannot use the provided interfaces.
// `pub extern crate` does not work. See https://github.com/rust-lang/rust/issues/21757
// This is why import crate for a different name here such that we can use pub use below.
pub extern crate proto; 

extern crate zookeeper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

mod master_detector;
pub mod scheduler;
pub mod scheduler_driver;
mod executor;
mod executor_driver;
#[allow(non_upper_case_globals)]
mod http_api;
