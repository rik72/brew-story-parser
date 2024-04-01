use crate::parsing::parsed::one_action_parsed::OneActionParsed;

use super::{parse_error::ParseError, parseable::Parseable, parser_service::PARSER};

pub struct OneActionParser;

impl OneActionParser {
    const ONE_ACTION_PATTERN: &'static str = r"^word *\. *([text])?$";
    pub const ONE_ACTION_HR: &'static str = "verb . [ (feedback) ]?";

    pub fn new() -> Self {
        PARSER
            .service_mut()
            .unwrap()
            .register_pattern(Self::ONE_ACTION_PATTERN);
        Self
    }
}

impl Parseable<String, OneActionParsed> for OneActionParser {
    fn parse(&self, raw: &mut String) -> Result<Option<OneActionParsed>, ParseError> {
        let mut captures: Vec<Option<String>> = Vec::new();
        PARSER.service().unwrap().capture_pattern(
            Self::ONE_ACTION_PATTERN.to_string(),
            raw.trim().to_string(),
            &mut captures,
        );
        if captures.len() > 0 {
            let verb_match = &captures[1];
            let feedback_match = &captures[3];
            let verb_str = match verb_match {
                Some(str) => str.trim().to_string(),
                None => {
                    return Err(ParseError::InvalidFormat(
                        raw.to_string(),
                        Self::ONE_ACTION_HR.to_string(),
                    ));
                }
            };
            return Ok(Some(OneActionParsed {
                verb: verb_str,
                feedback: match feedback_match {
                    Some(feedback_str) => Some(feedback_str.to_string()),
                    None => None,
                },
            }));
        }

        Ok(None)
    }
}
