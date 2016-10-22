#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod dbms;
pub mod config;
