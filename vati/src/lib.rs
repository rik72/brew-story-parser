#[macro_use]
extern crate rust_i18n;

i18n!("../locales", fallback = "en");

pub mod core;
pub mod parse_locale;
