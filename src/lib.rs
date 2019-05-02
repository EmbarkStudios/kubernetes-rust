#![feature(await_macro, async_await)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod config;
mod oauth2;
