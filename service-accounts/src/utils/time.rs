use chrono::Utc;

pub fn get_current_timestamp() -> String {
    Utc::now().timestamp().to_string()
}
