use std::collections::HashMap;

use crate::core::vati_locale_service::VATI_LOCALE;

use super::parse_locale::ParseLocale;

pub struct ParseLocalized<T> {
    value_map: HashMap<ParseLocale, T>,
}

impl<T> ParseLocalized<T> {
    pub fn new(ascii_value: T, japanese_value: T) -> ParseLocalized<T> {
        let mut value_map = HashMap::new();
        value_map.insert(ParseLocale::Ascii, ascii_value);
        value_map.insert(ParseLocale::Japanese, japanese_value);
        Self { value_map }
    }

    pub fn get(&self) -> &T {
        self.value_map
            .get(&VATI_LOCALE.service().unwrap().get_current_parse_locale())
            .unwrap()
    }

    pub fn get_by_parse_locale(&self, parse_locale: &ParseLocale) -> &T {
        self.value_map.get(parse_locale).unwrap()
    }
}
