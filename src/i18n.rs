const SUPPORTED_LANGUAGES: [&str; 17] = [
    "en", "zh_CN", "zh_TW", "fr", "ja", "es", "de", "ru", "it", "pt", "ko", "no", "is", "uk", "th",
    "el", "da",
];
pub fn set_locale(language: &Option<String>) {
    match language {
        Some(lang) => {
            let lang = if lang == "zh" { "zh_CN" } else { lang };
            if SUPPORTED_LANGUAGES.contains(&lang) {
                rust_i18n::set_locale(lang);
            } else {
                rust_i18n::set_locale("en");
            }
        }
        None => match get_language() {
            Some(lang) => {
                rust_i18n::set_locale(lang.as_str());
            }
            None => {
                rust_i18n::set_locale("en");
            }
        },
    }
}
fn get_language() -> Option<String> {
    match std::env::var("LANG") {
        Ok(lang) => {
            let language = lang.split('.').next();
            language.map(|s| s.to_string())
        }
        Err(_) => None,
    }
}
