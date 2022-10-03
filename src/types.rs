/// this MUST align withe HttpCurl defined in the smashnet main package!
pub struct Curler {
    pub callback: Option<fn(f64, f64) -> ()>,
    pub curl: u64,
}

pub trait HttpCurl {
    fn new() -> Self;
    fn is_valid(&mut self) -> Result<&mut Self, u64>;
    fn download(&mut self, url: String, location: String) -> Result<(), u32>;    
    fn get(&mut self, url: String) -> Result<String, String>;
    fn get_json(&mut self, url: String) -> Result<String, String>;
    fn progress_callback(&mut self, callback: fn(f64, f64) -> ()) -> &mut Self;
}