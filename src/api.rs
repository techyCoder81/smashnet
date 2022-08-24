use crate::types::*;

extern "Rust" {
    // HttpCurl
    #[link_name = "HttpCurl__new"]
    fn HttpCurl__new() -> Curler;
    #[link_name = "HttpCurl__is_valid"]
    fn HttpCurl__is_valid(curler: &Curler) -> Result<&mut Curler, u64>;
    #[link_name = "HttpCurl__download"]
    fn HttpCurl__download(curler: &Curler, url: String, location: String) -> Result<(), u32>;
    #[link_name = "HttpCurl__progress_callback"]
    fn HttpCurl__progress_callback(curler: &Curler, callback: fn(f64, f64) -> ()) -> &mut Curler;

    // Drop for Curler
    #[link_name = "Curler__drop"]
    fn Curler__drop(curler: &Curler);
}

impl HttpCurl for Curler {
    fn new() -> Self {
        unsafe {
            HttpCurl__new()
        }
    }
    fn is_valid(&mut self) -> Result<&mut Self, u64> {
        unsafe {
            HttpCurl__is_valid(self)
        }
    }
    fn download(&mut self, url: String, location: String) -> Result<(), u32> {
        unsafe {
            HttpCurl__download(self, url, location)
        }
    }
    fn progress_callback(&mut self, callback: fn(f64, f64) -> ()) -> &mut Self {
        unsafe {
            HttpCurl__progress_callback(self, callback)
        }
    }
}

impl Drop for Curler {
    fn drop(&mut self) {
        unsafe {
            Curler__drop(self)
        }
    }
}