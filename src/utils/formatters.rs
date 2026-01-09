pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn format_expires(expires_at: &str) -> String {
    expires_at
        .split('T')
        .next()
        .unwrap_or(expires_at)
        .to_string()
}

#[allow(unused)]
pub fn format_datetime(datetime: &str) -> String {
    if let Some(date_part) = datetime.split('T').next() {
        if let Some(time_part) = datetime.split('T').nth(1) {
            let time = time_part.split('.').next().unwrap_or("");
            return format!("{} às {}", date_part, time);
        }
        return date_part.to_string();
    }
    datetime.to_string()
}
