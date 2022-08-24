#![feature(proc_macro_hygiene)]
#![feature(allocator_api)]
//#![feature(asm)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(c_variadic)]
use std::{io::{BufWriter, Write}, fs::File, path::Path};
use crate::curl::*;

mod curl;

#[skyline::main(name = "smashnet")]
pub fn main() -> Result<(), u64> {
    curl::install();
    
    println!("starting main for smashnet");
    
    let path = format!("sd:/content_hashes.txt");
    let url = format!("https://github.com/HDR-Development/HDR-Releases/releases/download/v0.18.3/content_hashes.txt");
    match Curler::new()
        .is_valid()?
        .progress_callback(
            |total, progress| println!("Progress: {}", progress/total)
        )
        .download(url, path) {
            Ok(()) => println!("download successful!"),
            Err(e) => println!("download failed with error code: {}", e)
        };

    println!("Smashnet main has run.");
    Ok(())
}
