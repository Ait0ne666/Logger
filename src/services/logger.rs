use chrono::prelude::*;
use std::{time::SystemTime, fmt::Debug};

use crate::{setup_grpc::App};
use crate::prelude::*;


pub trait Logging {
    fn log(&'static self, message: &str, severity: Severity, app: App); 
}



pub enum Severity {
    Warning,
    Info,
    Error
}



impl From<&str> for Severity {
    
    
    
    fn from(s: &str) -> Self {
        match s {
            "Error" => {
                Severity::Error
            }
            "Warning" => {
                Severity::Warning
            }
            "Info" => {
                Severity::Info
            }
            _ => {
                Severity::Info
            }
        }
    }
}

impl From<String> for Severity {
    
    
    
    fn from(s: String) -> Self {
        match s.as_str() {
            "Error" => {
                Severity::Error
            }
            "Warning" => {
                Severity::Warning
            }
            "Info" => {
                Severity::Info
            }
            _ => {
                Severity::Info
            }
        }
    }
}


impl Into<&str> for Severity {
    fn into(self) -> &'static str {
        match self {
            Severity::Warning => {
                "WARNING"
            },
            Severity::Info => {
                "INFO"
            },
            Severity::Error => {
                "ERROR"
            },
        }
    }
}


#[derive(Debug, Default, Clone)]
pub struct Logger {}




impl Logging for Logger {
     fn log(&self, message: &str, severity: Severity, app: App) {
        let time = Utc::now();
        let timestamp = time.format("%d-%m-%Y %H:%M:%S").to_string();

        let s: &str = severity.into();

        let formatted_message = format!("[{}]-{}\nApp: {}]\nMessage: {}", timestamp, s, app.title, message); 


        send_message_to_telegram(&formatted_message, app.telegram_chat_id);
    }
}

