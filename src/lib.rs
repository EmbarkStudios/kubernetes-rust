#![feature(await_macro, async_await)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;

pub mod client;
pub mod config;
