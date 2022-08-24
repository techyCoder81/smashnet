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
    curl::install();
    
    println!("starting main for smashnet");
    
    let location = format!("sd:/content_hashes.txt");
    let url = format!("https://github.com/HDR-Development/HDR-Releases/releases/download/v0.18.3/content_hashes.txt");
    match Curler::new()
        .is_valid()?
        .progress_callback(
            |total, current| println!("Progress: {}", current/total)
        )
        .download(url, location) {
            Ok(()) => println!("download successful!"),
            Err(e) => println!("download failed with error code: {}", e)
        };

    println!("Smashnet main has run.");
    Ok(())
}
