use chrono::prelude::*;

pub fn get_current_timestamp() -> String {
    Utc::now().to_rfc3339()
}