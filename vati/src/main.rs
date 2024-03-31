use rust_i18n::t;
use vati::{
    _rust_i18n_translate,
    core::{
        translations_service::{TranslationsServiceProvider, TRANSLATIONS},
        vati_locale_service::{VatiLocaleServiceProvider, VATI_LOCALE},
    },
    parse_locale::{parse_locale::ParseLocale, parse_localized::ParseLocalized},
};

fn main() {
    TRANSLATIONS.provide(Box::new(TranslationsServiceProvider::new()));
    VATI_LOCALE.provide(Box::new(VatiLocaleServiceProvider::new()));

    rust_i18n::set_locale("en");
    println!("{}", t!("begin_adventuring"));
    rust_i18n::set_locale("ja");
    println!("{}", t!("begin_adventuring"));

    println!(
        "{:?}",
        VATI_LOCALE.service().unwrap().get_current_parse_locale()
    );

    let localized_str: ParseLocalized<String> =
        ParseLocalized::new("english".to_string(), "日本語".to_string());

    println!("{}", localized_str.get());
    println!(
        "{}",
        localized_str.get_by_parse_locale(&ParseLocale::Japanese)
    );
}
