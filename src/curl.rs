use std::{io::{BufWriter, Write}, fs::*, fs, sync::mpsc::Sender};
use skyline::libc::*;
use std::arch::asm;
use std::error::Error;
use std::path::Path;
use crate::*;
use crate::curl_consts::HandleCode;

pub struct CurlHandle;

pub struct curl_slist {
    pub data: *mut c_char,
    pub next: *mut curl_slist,
}

//#[skyline::hook(offset = 0x6aa8, inline)]
//pub unsafe fn curl_log_hook(ctx: &skyline::hooks::InlineCtx) {
    //let str_ptr;
    //asm!("ldr {}, [x29, #0x18]", out(reg) str_ptr);
    // println!("{}", skyline::from_c_str(str_ptr));
//}

#[skyline::hook(offset = 0x27ac0, inline)]
pub unsafe fn libcurl_resolver_thread_stack_size_set(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[1].set_x(0x10_000);
}

#[skyline::hook(offset = 0x27af4, inline)]
pub unsafe fn libcurl_resolver_thread_stack_size_set2(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[4].set_x(0x10_000);
}


#[skyline::from_offset(0x7f0)]
pub unsafe extern "C" fn global_init_mem(
    init_args: u64,
    malloc: unsafe extern "C" fn(usize) -> *mut c_void,
    free: unsafe extern "C" fn(*mut c_void),
    realloc: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void,
    strdup: unsafe extern "C" fn(*const u8) -> *mut u8,
    calloc: unsafe extern "C" fn(usize, usize) -> *mut c_void
) -> u64;

#[skyline::from_offset(0x16c00)]
pub unsafe extern "C" fn slist_append(slist: *mut curl_slist, header: *const u8) -> u64;

#[skyline::from_offset(0x960)]
pub unsafe extern "C" fn easy_init() -> *mut CurlHandle;

#[skyline::from_offset(0xA00)]
pub unsafe extern "C" fn easy_setopt(curl: *mut CurlHandle, option: u32, ...) -> u32;

#[skyline::from_offset(0xA90)]
pub unsafe extern "C" fn easy_perform(curl: *mut CurlHandle) -> u32;

#[skyline::from_offset(0xC70)]
pub unsafe extern "C" fn easy_cleanup(curl: *mut CurlHandle) -> u32;

#[skyline::from_offset(0x36f7dc0)]
pub unsafe extern "C" fn curl_global_malloc(size: usize) -> *mut u8;

#[skyline::from_offset(0x36f7e40)]
pub unsafe extern "C" fn curl_global_free(ptr: *mut u8);

#[skyline::from_offset(0x36f7ec0)]
pub unsafe extern "C" fn curl_global_realloc(ptr: *mut u8, size: usize) -> *mut u8;

#[skyline::from_offset(0x36f7f40)]
pub unsafe extern "C" fn curl_global_strdup(ptr: *const u8) -> *mut u8;

#[skyline::from_offset(0x36f8020)]
pub unsafe extern "C" fn curl_global_calloc(nmemb: usize, size: usize) -> *mut u8;

#[skyline::from_offset(0x21fd50)]
pub unsafe extern "C" fn curl_ssl_ctx_callback(arg1: u64, arg2: u64, arg3: u64) -> u64;

unsafe extern "C" fn write_fn(data: *const u8, data_size: usize, data_count: usize, writer: &mut BufWriter<File>) -> usize {
    let true_size = data_size * data_count;
    let slice = std::slice::from_raw_parts(data, true_size);
    let _ = writer.write(slice);
    true_size
}

unsafe extern "C" fn write_fn_data(data: *const u8, data_size: usize, data_count: usize, writer: &mut BufWriter<&mut Vec<u8>>) -> usize {
    let true_size = data_size * data_count;
    let slice = std::slice::from_raw_parts(data, true_size);
    let _ = writer.write(slice);
    true_size
}
/// private internal callback handler
unsafe extern "C" fn callback_internal(curler: &Curler, dl_total: f64, dl_now: f64, ul_total: f64, ul_now: f64) -> usize {
    //println!("callback is called: {:p}", callback);
    if dl_total != 0.0 && (dl_now/dl_total * 1000.0).trunc()/10.0 == (dl_now/dl_total * 100.0).trunc()  {
        (curler.callback.unwrap())(dl_total, dl_now);
    }
    0
}

macro_rules! curle {
    ($e:expr) => {{
        let result = $e;
        if result != curl_consts::HandleCode::CURLE_OK as u32 {
            Err(result)
        } else {
            Ok(())
        }
    }}
}


pub struct Curler<'c> {
    pub callback: Option<&'c dyn Fn(f64, f64) -> ()>,
    pub curl: u64,
}

impl<'a, 'c> Curler<'c> {
    /// creates a new curler
    pub extern "Rust" fn new() -> Self { 
        install_curl();
        let curl_handle = unsafe { easy_init() };
        return Curler{callback: None, curl: curl_handle as u64};
    }

    /// download a file from the given url to the given location
    pub extern "Rust" fn download(&mut self, url: String, location: String) -> Result<(), u32>{
        // change thread to high priority
        //unsafe {
        //    skyline::nn::os::ChangeThreadPriority(skyline::nn::os::GetCurrentThread(), 2);
        //}

        // temp file name: myfile.txt.dl
        let temp_file = [location.as_str(), ".dl"].concat();
        if Path::new(temp_file.as_str()).exists() {
            // println!("removing existing temp file: {}", temp_file);
            std::fs::remove_file(&temp_file);
        }

        // println!("creating temp file: {}", temp_file);
        let mut writer = std::io::BufWriter::with_capacity(
            0x40_0000,
            std::fs::File::create(&temp_file).unwrap()
        );
        // println!("created bufwriter with capacity");
        unsafe {
            let cstr = [url.as_str(), "\0"].concat();
            let ptr = cstr.as_str().as_ptr();
            let curl = self.curl as *mut CurlHandle;
            // println!("curl is initialized, beginning options");
            //let header = slist_append(std::ptr::null_mut(), "Accept: application/octet-stream\0".as_ptr());
            curle!(easy_setopt(curl, curl_consts::CURLOPT_URL, ptr))?;
            //curle!(easy_setopt(curl, curl_consts::CURLOPT_HTTPHEADER, header))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_FOLLOWLOCATION, 1u64))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_WRITEDATA, &mut writer))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_WRITEFUNCTION, write_fn as *const ()))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_FAILONERROR, 1u64))?;
       
            match self.callback {
                Some(function) => {
                    curle!(easy_setopt(curl, curl_consts::CURLOPT_NOPROGRESS, 0u64))?;
                    curle!(easy_setopt(curl, curl_consts::CURLOPT_PROGRESSDATA, self as *const Curler))?;
                    curle!(easy_setopt(curl, curl_consts::CURLOPT_PROGRESSFUNCTION, callback_internal as *const ()))?;
                },
                None => curle!(easy_setopt(curl, curl_consts::CURLOPT_NOPROGRESS, 1u64))?,
            }
            curle!(easy_setopt(curl, curl_consts::CURLOPT_NOSIGNAL, 1u64))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_SSL_CTX_FUNCTION, curl_ssl_ctx_callback as *const ()))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_USERAGENT, "smashnet\0".as_ptr()))?;
            // println!("beginning download.");
            let result = curle!(easy_perform(curl));
            match result {
                Ok(()) => println!("curl success?"),
                Err(e) => {
                    println!("Error during curl: {}", e);
                    println!("flushing writer");
                    writer.flush();
                    println!("dropping writer");
                    std::mem::drop(writer);
                    match fs::metadata(&temp_file.as_str()) {
                        Ok(data) => {
                            std::fs::remove_file(&temp_file);
                        }
                        Err(e) => println!("Error while checking for temp file, after failed download: {:?}", e)
                    }
                    return Err(e);
                }
            };
        }

        // println!("flushing writer");
        writer.flush();
        // println!("dropping writer");
        std::mem::drop(writer);

        // replace/rename the temp file to the expected location
        if Path::new(location.as_str()).exists() {
            // println!("removing original path: {}", location);
            std::fs::remove_file(location.as_str());
        }
        std::fs::rename(&temp_file, location);

        //println!("resetting priority of thread");
        //unsafe {
        //    skyline::nn::os::ChangeThreadPriority(skyline::nn::os::GetCurrentThread(), 16);
        //}
        // println!("download complete.");
        Ok(())
    }

    /// GET text from the given url
    pub extern "Rust" fn get(&mut self, url: String) -> Result<String, String>{
        let tick = unsafe {skyline::nn::os::GetSystemTick() as usize};
        fs::create_dir_all("sd:/downloads");
        let location = format!("sd:/downloads/{}.txt", tick);
        match self.download(url, location.clone()) {
            Ok(()) => (),//println!("text GET ok!"),
            Err(e) => {
                let error = format!("{}", e);
                return Err(error);
            }
        }
        let str = match std::fs::read_to_string(&location){
            Ok(text) => text,
            Err(e) => {
                let error = format!("{}", e);
                return Err(error);
            }
        };
        std::fs::remove_file(&location);
        return Ok(str);
    }

    pub extern "Rust" fn get_bytes(&mut self, url: String, out_buf: &mut Vec<u8>) -> Result<(), u32> {
        let mut writer = BufWriter::with_capacity(0x40_000,  out_buf);
        unsafe {
            let cstr = [url.as_str(), "\0"].concat();
            let ptr = cstr.as_str().as_ptr();
            let curl = self.curl as *mut CurlHandle;
            // println!("curl is initialized, beginning options");
            let header = slist_append(std::ptr::null_mut(), "Accept: application/octet-stream\0".as_ptr());
            curle!(easy_setopt(curl, curl_consts::CURLOPT_URL, ptr))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_HTTPHEADER, header))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_FOLLOWLOCATION, 1u64))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_WRITEDATA, &mut writer))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_WRITEFUNCTION, write_fn_data as *const ()))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_FAILONERROR, 1u64))?;
       
            match self.callback {
                Some(ref function) => {
                    curle!(easy_setopt(curl, curl_consts::CURLOPT_NOPROGRESS, 0u64))?;
                    curle!(easy_setopt(curl, curl_consts::CURLOPT_PROGRESSDATA, self as *const Curler))?;
                    curle!(easy_setopt(curl, curl_consts::CURLOPT_PROGRESSFUNCTION, callback_internal as *const ()))?;
                },
                None => curle!(easy_setopt(curl, curl_consts::CURLOPT_NOPROGRESS, 1u64))?,
            }
            curle!(easy_setopt(curl, curl_consts::CURLOPT_NOSIGNAL, 1u64))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_SSL_CTX_FUNCTION, curl_ssl_ctx_callback as *const ()))?;
            curle!(easy_setopt(curl, curl_consts::CURLOPT_USERAGENT, "smashnet\0".as_ptr()))?;
            //println!("beginning download.");
            match curle!(easy_perform(curl)){
                Ok(()) => (), // println!("curl success?"),
                Err(e) => {
                    println!("Error during curl: {}", e);
                    return Err(e);
                } 
            };
        }

        // println!("flushing writer");
        writer.flush();
        return Ok(());

    }

    pub extern "Rust" fn progress_callback(&mut self, function: &'a (impl Fn(f64, f64) -> () + 'a)) -> &mut Self where 'a: 'c {
        self.callback = Some(function);
        self
    }
}

impl <'c>Drop for Curler<'c> {
    extern "Rust" fn drop(&mut self) {
        let curl = self.curl as *mut CurlHandle;
        if !curl.is_null() {
            // println!("cleaning up curl handle from curler.");
            unsafe { 
                match curle!(easy_cleanup(curl)) {
                    Ok(_) => (), //println!("cleaned up curl successfully."),
                    Err(e) => println!("cleaning up curl failed with error code: {}", e),
                }; 
            }
        }
    }
}

static mut INSTALLED: bool = false;

pub fn install_curl() {
    unsafe {
        if !INSTALLED {
            INSTALLED = true;
            skyline::install_hooks!(
                //curl_log_hook,
                libcurl_resolver_thread_stack_size_set,
                libcurl_resolver_thread_stack_size_set2
            );
        }
    }
}