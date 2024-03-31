use crate::parse_locale::parse_locale::ParseLocale;

pub enum VatiLocale {
    English,
    Japanese,
}

impl VatiLocale {
    pub fn locale(&self) -> String {
        match self {
            VatiLocale::English => "en".to_string(),
            VatiLocale::Japanese => "ja".to_string(),
        }
    }

    pub fn parse_locale(&self) -> ParseLocale {
        match self {
            VatiLocale::English => ParseLocale::Ascii,
            VatiLocale::Japanese => ParseLocale::Japanese,
        }
    }

    pub fn from(str: &str) -> Option<VatiLocale> {
        if "english".eq(str) {
            return Some(VatiLocale::English);
        } else if "japanese".eq(str) {
            return Some(VatiLocale::Japanese);
        }
        None
    }
}
