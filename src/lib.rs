#![feature(proc_macro_hygiene)]
#![feature(allocator_api)]
//#![feature(asm)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(c_variadic)]
use std::{io::{BufWriter, Write}, fs::File, path::Path};

#[cfg(not(feature = "nro"))]
pub mod api;

mod types;
pub use types::HttpCurl;
#[cfg(not(feature = "nro"))]
pub use types::Curler;

// this package is the plugin api (the extern api)

#[cfg(not(feature = "nro"))]
pub use api::*;
