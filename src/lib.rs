#![feature(proc_macro_hygiene)]
#![feature(allocator_api)]
//#![feature(asm)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(c_variadic)]
use std::{io::{BufWriter, Write}, fs::File, path::Path};

pub mod curl;
pub mod curl_consts;
