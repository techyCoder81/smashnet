#![feature(proc_macro_hygiene)]
#![feature(allocator_api)]
//#![feature(asm)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(c_variadic)]
use std::{io::{BufWriter, Write}, fs::File, path::Path};

// this package is the curl implementation
#[cfg(not(feature = "dynamic"))]
use crate::curl::*;

// use types no matter what
use crate::types::*;

// this package is the plugin api (the extern api)
#[cfg(feature = "dynamic")]
pub mod api;
#[cfg(feature = "dynamic")]
use crate::api::*;

#[cfg(not(feature = "dynamic"))]
mod curl;

pub mod types;

#[cfg(not(feature = "dynamic"))]
#[skyline::main(name = "smashnet")]
pub fn main() -> Result<(), u64> {
    
    println!("starting main for smashnet");
    
    println!("Smashnet main has run.");
    Ok(())
}
