use crate::parsing::parsed::direction_parsed::DirectionParsed;

use super::{parse_error::ParseError, parseable::Parseable, parser_service::PARSER};

const DIRECTION_PATTERN: &str = "^word *([word])?$";
const DIRECTION_HR: &str = "location [ (verb) ]?";

pub struct DirectionParser;

impl DirectionParser {
    pub fn new() -> Self {
        PARSER
            .service_mut()
            .unwrap()
            .register_pattern(DIRECTION_PATTERN);
        Self
    }
}

impl Parseable<Option<String>, DirectionParsed> for DirectionParser {
    fn parse(&self, raw: &mut Option<String>) -> Result<Option<DirectionParsed>, ParseError> {
        if let Some(direction) = raw.take() {
            let mut captures: Vec<Option<String>> = Vec::new();
            PARSER.service().unwrap().capture_pattern(
                DIRECTION_PATTERN.to_string(),
                direction.trim().to_string(),
                &mut captures,
            );
            if captures.len() > 0 {
                let location_match = &captures[1];
                let verb_match = &captures[3];
                let location_str = match location_match {
                    Some(str) => str.trim().to_string(),
                    None => {
                        return Err(ParseError::InvalidFormat(
                            direction,
                            DIRECTION_HR.to_string(),
                        ));
                    }
                };
                return Ok(Some(DirectionParsed {
                    location: location_str,
                    verb: if let Some(verb) = verb_match {
                        Some(verb.to_string())
                    } else {
                        None
                    },
                }));
            } else {
                return Ok(None);
            };
        }

        Ok(None)
    }
}
