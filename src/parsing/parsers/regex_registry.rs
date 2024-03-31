use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use vati::parse_locale::parse_localized::ParseLocalized;

pub struct RegexRegistry {
    regex_map: HashMap<String, ParseLocalized<Regex>>,
}

impl RegexRegistry {
    pub fn new() -> Self {
        Self {
            regex_map: HashMap::new(),
        }
    }

    pub fn contains_key(&mut self, pattern: &str) -> bool {
        self.regex_map.contains_key(pattern)
    }

    pub fn entry(&mut self, pattern: String) -> Entry<String, ParseLocalized<Regex>> {
        self.regex_map.entry(pattern)
    }

    pub fn get(&self, pattern: String) -> Option<&ParseLocalized<Regex>> {
        self.regex_map.get(&pattern)
    }

    pub fn insert(&mut self, pattern: String, localized_regex: ParseLocalized<Regex>) {
        self.regex_map.insert(pattern, localized_regex);
    }
}
