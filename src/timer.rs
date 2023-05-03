use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;
use std::string::ToString;
use std::thread::sleep;
use std::time::{Duration, Instant};

use winapi::um::winuser::{MessageBoxW, MB_OK};
use std::ptr;

const POPUP_EVERY_N_SEC: u64 = 300;


pub fn inf_popup() {
    let mut start = Instant::now();

    loop {
        sleep(Duration::from_secs(1));

        if start.elapsed() >= Duration::from_secs(POPUP_EVERY_N_SEC) {
            unsafe { MessageBoxW(ptr::null_mut(), encode("混雑しているので、1人5分程度で交代してください。").as_ptr(), encode("お知らせ").as_ptr(), MB_OK); }
            
            start = Instant::now();
        }
    }
}

fn encode(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
