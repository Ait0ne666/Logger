use std::env;
use url::form_urlencoded::byte_serialize;

pub async fn send_message_to_telegram(message: &str, chat_id: Option<String>) {
    let mut default_chat = get_default_chat();

    match chat_id {
        Some(id) => {
            default_chat = id;
        }
        None => {}
    }

    let url_encoded: String = byte_serialize(message.to_owned().as_bytes()).collect();
    let token = get_chat_token();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        token, default_chat, url_encoded
    );
    let resp = reqwest::get(url).await;

    match resp {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn get_chat_token() -> String {
    let maybe_token = env::var("TG_TOKEN");

    match maybe_token {
        Ok(t) => t,
        Err(_) => "".to_string(),
    }
}

fn get_default_chat() -> String {
    let maybe_token = env::var("CHAT_ID");

    match maybe_token {
        Ok(t) => t,
        Err(_) => "".to_string(),
    }
}
