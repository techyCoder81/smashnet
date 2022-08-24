pub struct Curler {
    pub callback: Option<fn(f64, f64) -> ()>,
    pub curl: u64,
}

trait HttpCurl {
    fn new() -> Self;
    fn is_valid(&mut self) -> Result<&mut Self, u64>;
    fn download(&mut self, url: String, location: String) -> Result<(), u32>;
    fn progress_callback(&mut self, callback: fn(f64, f64) -> ()) -> &mut Self;
}