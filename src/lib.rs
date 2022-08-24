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
    curl::install();
    
    println!("starting main for smashnet");
    unsafe {
        skyline::nn::os::ChangeThreadPriority(skyline::nn::os::GetCurrentThread(), 2);
    }
    let path = "sd:/download_test.txt";
    // delete the original file if this file already exists on sd
    if Path::new(path).exists() {
        println!("removing original path: {}", path);
        std::fs::remove_file(path);
    }
    let mut writer = std::io::BufWriter::with_capacity(
        0x40_0000,
        std::fs::File::create(path).unwrap()
    );
    println!("created bufwriter with capacity");
    let url = format!("https://github.com/HDR-Development/HDR-Nightlies/releases/download/v0.19.1/hdr_version.txt").as_str();
    println!("Curl output: {}", match curl::try_curl_maidenless(url, &mut writer) {
        Ok(()) => "curl was OK!".to_string(),
        Err(e) => format!("Failed output: {:?}", e)
    });
    println!("flushing writer");
    writer.flush();
    println!("dropping writer");
    std::mem::drop(writer);
    println!("resetting priority of thread");
    unsafe {
        skyline::nn::os::ChangeThreadPriority(skyline::nn::os::GetCurrentThread(), 16);
    }
    println!("Smashnet main has run.");
}
