use crate::parsing::parsed::consequence_parsed::ConsequenceParsed;

use super::{parse_error::ParseError, parseable::Parseable, parser_service::PARSER};

const CONSEQUENCE_PATTERN: &str = r"^(\.|([word]) *word) *(-> *word)? *(=> *word)? *([text])?$";
pub const CONSEQUENCE_HR: &str =
    "( . | (thing) status_before [ -> status_after ]? [ => location_after ]? [ (feedback) ]? )";

pub struct ConsequenceParser {}

impl ConsequenceParser {
    pub fn new() -> Self {
        PARSER
            .service_mut()
            .unwrap()
            .register_pattern(CONSEQUENCE_PATTERN);
        Self {}
    }
}

impl Parseable<String, ConsequenceParsed> for ConsequenceParser {
    fn parse(&self, raw: &mut String) -> Result<Option<ConsequenceParsed>, ParseError> {
        let mut captures: Vec<Option<String>> = Vec::new();
        PARSER.service().unwrap().capture_pattern(
            CONSEQUENCE_PATTERN.to_string(),
            raw.trim().to_string(),
            &mut captures,
        );
        if captures.len() > 0 {
            let entity_match = &captures[1];
            let canonical_match = &captures[3];
            let before_match = &captures[4];
            let after_match = &captures[6];
            let to_match = &captures[8];
            let feedback_match = &captures[10];
            let entity_str = match entity_match {
                Some(str) => str.trim().to_string(),
                None => "".to_string(),
            };
            let canonical_str = match canonical_match {
                Some(str) => str.trim().to_string(),
                None => "".to_string(),
            };
            let before_str = match before_match {
                Some(str) => str.trim().to_string(),
                None => "".to_string(),
            };
            let after_str = match after_match {
                Some(str) => str.trim().to_string(),
                None => "".to_string(),
            };
            let to_str = match to_match {
                Some(str) => str.trim().to_string(),
                None => "".to_string(),
            };

            if ".".eq(entity_str.as_str()) {
                return Ok(Some(ConsequenceParsed {
                    entity: None,
                    before: None,
                    after: after_str,
                    to: to_str,
                    feedback: match feedback_match {
                        Some(feedback_str) => Some(feedback_str.to_string()),
                        None => None,
                    },
                }));
            } else {
                return Ok(Some(ConsequenceParsed {
                    entity: Some(canonical_str),
                    before: Some(before_str),
                    after: after_str,
                    to: to_str,
                    feedback: match feedback_match {
                        Some(feedback_str) => Some(feedback_str.to_string()),
                        None => None,
                    },
                }));
            }
        }

        Err(ParseError::InvalidFormat(
            raw.to_string(),
            format!("`{}`", CONSEQUENCE_HR.to_string()),
        ))
    }
}
