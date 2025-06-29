use base64::{engine::general_purpose, Engine as _};


// ───── IMAGES ─────
const FAVICON_BYTES: &[u8] = include_bytes!("../assets/favicon.ico");
const HEADER_BYTES: &[u8] = include_bytes!("../assets/header.png");

pub fn favicon_data() -> String {
    format!("data:image/x-icon;base64,{}", general_purpose::STANDARD.encode(FAVICON_BYTES))
}

pub fn header_data() -> String {
    format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(HEADER_BYTES))
}


// ───── STYLES ─────
const MAIN_CSS_STR: &str = include_str!("../assets/main.css");
const TAILWIND_CSS_STR: &str = include_str!("../assets/tailwind.css");

pub fn main_css() -> &'static str {
    MAIN_CSS_STR
}

pub fn tailwind_css() -> &'static str {
    TAILWIND_CSS_STR
}
