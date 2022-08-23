#![feature(proc_macro_hygiene)]
#![feature(allocator_api)]
#![feature(asm)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(c_variadic)]
use std::{io::{BufWriter}, fs::File};

mod curl;

#[skyline::main(name = "smashnet")]
pub fn main() {
    println!("starting main for smashnet");
    let mut writer = std::io::BufWriter::with_capacity(
        0x40_0000,
        std::fs::File::create("sd:/download_test.txt").unwrap()
    );
    println!("created bufwriter with capacity");
    let url = "https://github.com/HDR-Development/HDR-Nightlies/releases/download/v0.19.1/hdr_version.txt";
    println!("Curl output: {}", match curl::try_curl_maidenless(url, &mut writer) {
        Ok(()) => "curl was OK!".to_string(),
        Err(e) => format!("Failed output: {:?}", e)
    });
    println!("Smashnet main has run.");
}
