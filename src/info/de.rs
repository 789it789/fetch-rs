pub fn name() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "N/A".to_string())
}
