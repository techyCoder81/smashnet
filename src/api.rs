use crate::*;

extern "Rust" {
    // HttpCurl
    #[link_name = "HttpCurl__new"]
    fn HttpCurl__new() -> Curler;
    #[link_name = "HttpCurl__is_valid"]
    fn HttpCurl__is_valid(&mut curler: Curler) -> Result<&mut Curler, u64>;
    #[link_name = "HttpCurl__download"]
    fn HttpCurl__download(&mut curler: Curler, url: String, location: String) -> Result<(), u32>;
    #[link_name = "HttpCurl__progress_callback"]
    fn HttpCurl__progress_callback(&mut curler: Curler, callback: fn(f64, f64) -> ()) -> &mut Curler;

    // Drop for Curler
    #[link_name = "Curler__drop"]
    fn Curler__drop(&mut curler: Curler);
}

impl HttpCurl for Curler {
    fn new() -> Self {
        HttpCurl__new()
    }
    fn is_valid(&mut self) -> Result<&mut Self, u64> {
        HttpCurl__is_valid(self)
    }
    fn download(&mut self, url: String, location: String) -> Result<(), u32> {
        HttpCurl__download(self, url, location)
    }
    fn progress_callback(&mut self, callback: fn(f64, f64) -> ()) -> &mut Self {
        HttpCurl__progress_callback(self, callback)
    }
}

impl Drop for Curler {
    fn drop(&mut self) {
        Curler__drop(self)
    }
}