use rust_i18n::i18n;

pub fn set_locale() {
    i18n!("locales", fallback = "en");
    match get_language() {
        Some(lang) => {
            rust_i18n::set_locale(lang.as_str());
        }
        None => {
            rust_i18n::set_locale("en");
        }
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
