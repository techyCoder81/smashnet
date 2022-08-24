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
#[cfg(feature = "nro")]
mod curl;
pub mod types;

// this package is the curl implementation
#[cfg(feature = "nro")]
pub use curl::*;

// use types no matter what
pub use types::*;

// this package is the plugin api (the extern api)

#[cfg(not(feature = "nro"))]
pub use api::*;
