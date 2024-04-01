use super::{parse_error::ParseError, regex_registry::RegexRegistry};
use crate::parsing::raw::traits::raw::Raw;
use regex::Regex;
use service_locator::ServiceLocator;
use services::service::Service;
use vati::parse_locale::{parse_locale::ParseLocale, parse_localized::ParseLocalized};

pub trait ParserService: Service {
    fn register_pattern(&mut self, pattern: &str);
    fn capture_pattern(&self, pattern: String, str: String, captures: &mut Vec<Option<String>>);
    fn check_not_empty_str(&self, entity_name: String, str: String) -> Result<(), ParseError>;
    fn check_not_empty_str_vec(
        &self,
        entity_name: String,
        str_vec: Vec<String>,
    ) -> Result<(), ParseError>;
    fn check_not_empty_raw_vec(
        &self,
        entity_name: String,
        raw_vec: Vec<&dyn Raw>,
    ) -> Result<(), ParseError>;
}

pub struct ParserServiceProvider {
    regex_registry: RegexRegistry,
    word_pattern_str: ParseLocalized<String>,
    text_pattern_str: ParseLocalized<String>,
}

impl ParserServiceProvider {
    pub fn new() -> Self {
        Self {
            regex_registry: RegexRegistry::new(),
            word_pattern_str: ParseLocalized::new(
                r" *([a-z \.]+) *".to_string(),
                r"　*(.+)　*".to_string(),
            ),
            text_pattern_str: ParseLocalized::new(
                r" *([^()]+) *".to_string(),
                r"　*([^「」]+)　*".to_string(),
            ),
        }
    }
}

impl Service for ParserServiceProvider {
    fn init(&mut self) {
        // NOP
    }
}

impl ParserService for ParserServiceProvider {
    fn register_pattern(&mut self, pattern: &str) {
        if !self.regex_registry.contains_key(pattern) {
            let asc = self.parse_locale_pattern(pattern.to_string(), ParseLocale::Ascii);
            let jap = self.parse_locale_pattern(pattern.to_string(), ParseLocale::Japanese);
            self.regex_registry
                .insert(pattern.to_string(), ParseLocalized::new(asc, jap));
        }
    }

    fn capture_pattern(&self, pattern: String, str: String, captures: &mut Vec<Option<String>>) {
        let regex = self.regex_registry.get(pattern).unwrap().get();
        // println!("      with regex `{}`", regex.as_str());
        if let Some(caps) = regex.captures(&str) {
            for cap in caps.iter() {
                match cap {
                    Some(capture) => captures.push(Some(capture.as_str().to_string())),
                    None => captures.push(None),
                }
            }
        }
    }

    fn check_not_empty_str(&self, entity_name: String, str: String) -> Result<(), ParseError> {
        if str.len() > 0 {
            return Ok(());
        }
        Err(ParseError::CheckFailed(entity_name))
    }

    fn check_not_empty_str_vec(
        &self,
        entity_name: String,
        str_vec: Vec<String>,
    ) -> Result<(), ParseError> {
        if str_vec.len() > 0 {
            return Ok(());
        }
        Err(ParseError::CheckFailed(entity_name))
    }

    fn check_not_empty_raw_vec(
        &self,
        entity_name: String,
        raw_vec: Vec<&dyn Raw>,
    ) -> Result<(), ParseError> {
        if raw_vec.len() > 0 {
            return Ok(());
        }
        Err(ParseError::CheckFailed(entity_name))
    }
}

impl ParserServiceProvider {
    fn parse_locale_pattern(&mut self, pattern: String, parse_locale: ParseLocale) -> Regex {
        Regex::new(
            pattern
                .replace(r"[", r"\(")
                .replace(r"]", r"\)")
                .replace(
                    r"word",
                    self.word_pattern_str.get_by_parse_locale(&parse_locale),
                )
                .replace(
                    r"text",
                    self.text_pattern_str.get_by_parse_locale(&parse_locale),
                )
                .as_str(),
        )
        .unwrap()
    }
}

// Provide the service.
pub static PARSER: ServiceLocator<dyn ParserService + Send + Sync> = ServiceLocator::new();
