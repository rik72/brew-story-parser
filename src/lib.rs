#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub mod common;
pub mod loading;
pub mod parsing;
pub mod services;
pub mod utils;
