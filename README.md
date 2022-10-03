# Smashnet
This is a crate which exposes GET request and download functionality for smash ultimate plugins. Instead of using minreq, which is slow due to lack of openssl impl for the switch, and which also requires devkitpro, this crate allows you to make get requests and download files using the game's own statically linked libcurl, which is offset hooked as necessary when `Curler::new()` is used.

## Example usage:
```rust
use smashnet::types::*;

#[skyline::main(name = "your-plugin-here")]
pub fn main() {
  Curler::new()
    .progress_callback(|progress, total| println!("Progress: {}", progress/total))
    .download("https://github.com/techyCoder81/smashnet-nro/releases/download/nightly/checksum.txt");
}
```
