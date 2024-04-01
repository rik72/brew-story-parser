use crate::parsing::parsed::two_action_parsed::TwoActionParsed;

use super::{parse_error::ParseError, parseable::Parseable, parser_service::PARSER};

const TWO_ACTION_PATTERN: &str = r"^word *\. *[word] *[word] *word *([text])?$";
pub const TWO_ACTION_HR: &str = "verb . (preposition) (supplement) status [ (feedback) ]?";

pub struct TwoActionParser {}

impl TwoActionParser {
    pub fn new() -> Self {
        PARSER
            .service_mut()
            .unwrap()
            .register_pattern(TWO_ACTION_PATTERN);
        Self {}
    }
}

impl Parseable<String, TwoActionParsed> for TwoActionParser {
    fn parse(&self, raw: &mut String) -> Result<Option<TwoActionParsed>, ParseError> {
        let mut captures: Vec<Option<String>> = Vec::new();
        PARSER.service().unwrap().capture_pattern(
            TWO_ACTION_PATTERN.to_string(),
            raw.trim().to_string(),
            &mut captures,
        );
        if captures.len() > 0 {
            let verb_match = &captures[1];
            let preposition_match = &captures[2];
            let supplement_match = &captures[3];
            let supplement_status_match = &captures[4];
            let feedback_match = &captures[6];
            let verb_str = match verb_match {
                Some(str) => str.trim().to_string(),
                None => {
                    return Err(ParseError::InvalidFormat(
                        raw.to_string(),
                        TWO_ACTION_HR.to_string(),
                    ));
                }
            };
            let preposition_str = match preposition_match {
                Some(str) => str.trim().to_string(),
                None => {
                    return Err(ParseError::InvalidFormat(
                        raw.to_string(),
                        TWO_ACTION_HR.to_string(),
                    ));
                }
            };
            let supplement_str = match supplement_match {
                Some(str) => str.trim().to_string(),
                None => {
                    return Err(ParseError::InvalidFormat(
                        raw.to_string(),
                        TWO_ACTION_HR.to_string(),
                    ));
                }
            };
            let supplement_status_str = match supplement_status_match {
                Some(str) => str.trim().to_string(),
                None => {
                    return Err(ParseError::InvalidFormat(
                        raw.to_string(),
                        TWO_ACTION_HR.to_string(),
                    ));
                }
            };
            return Ok(Some(TwoActionParsed {
                verb: verb_str,
                preposition: preposition_str,
                supplement: supplement_str,
                supplement_status: supplement_status_str,
                feedback: match feedback_match {
                    Some(feedback_str) => Some(feedback_str.to_string()),
                    None => None,
                },
            }));
        }

        Ok(None)
    }
}
