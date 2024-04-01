use crate::parsing::{
    parsed::possibility_parsed::PossibilityParsed, raw::possibility_raw::PossibilityRaw,
};

use super::{parse_error::ParseError, parseable::Parseable, parser_service::PARSER};

const POSSIBILITY_PATTERN: &str = "^word *([text])? *(!important)?$";
const POSSIBILITY_HR: &str = "verb [ (feedback) ]? [ !important ]?";
const POSSIBILITY_INHERIT_PATTERN: &str = "^[word] *word$";
const POSSIBILITY_INHERIT_HR: &str = "(status) verb";

pub struct PossibilityParser {}

impl PossibilityParser {
    pub fn new() -> Self {
        PARSER
            .service_mut()
            .unwrap()
            .register_pattern(POSSIBILITY_PATTERN);
        PARSER
            .service_mut()
            .unwrap()
            .register_pattern(POSSIBILITY_INHERIT_PATTERN);
        Self {}
    }
}

impl Parseable<PossibilityRaw, PossibilityParsed> for PossibilityParser {
    fn parse(&self, raw: &mut PossibilityRaw) -> Result<Option<PossibilityParsed>, ParseError> {
        if let Some(impossible) = raw.impossible.take() {
            let mut captures: Vec<Option<String>> = Vec::new();
            PARSER.service().unwrap().capture_pattern(
                POSSIBILITY_PATTERN.to_string(),
                impossible.trim().to_string(),
                &mut captures,
            );

            if captures.len() > 0 {
                let verb_match = &captures[1];
                let feedback_match = &captures[3];
                let important_match = &captures[4];
                match verb_match {
                    Some(verb) => {
                        let verb_str = verb.trim();
                        if verb_str.len() == 0 {
                            return Err(ParseError::InvalidFormat(
                                impossible,
                                POSSIBILITY_HR.to_string(),
                            ));
                        }
                        return Ok(Some(PossibilityParsed::Impossible {
                            verb: verb_str.to_string(),
                            feedback: match feedback_match {
                                Some(feedback) => Some(feedback.to_string()),
                                None => None,
                            },
                            important: match important_match {
                                Some(_) => true,
                                None => false,
                            },
                        }));
                    }
                    None => {
                        return Err(ParseError::InvalidFormat(
                            impossible,
                            POSSIBILITY_HR.to_string(),
                        ));
                    }
                }
            } else {
                return Ok(None);
            }
        }

        if let Some(possible) = raw.possible.take() {
            let mut captures: Vec<Option<String>> = Vec::new();
            PARSER.service().unwrap().capture_pattern(
                POSSIBILITY_PATTERN.to_string(),
                possible.trim().to_string(),
                &mut captures,
            );
            if captures.len() > 0 {
                let verb_match = &captures[1];
                let feedback_match = &captures[3];
                let important_match = &captures[4];
                match verb_match {
                    Some(verb) => {
                        let verb_str = verb.trim();
                        if verb_str.len() == 0 {
                            return Err(ParseError::InvalidFormat(
                                possible,
                                POSSIBILITY_HR.to_string(),
                            ));
                        }
                        return Ok(Some(PossibilityParsed::Possible {
                            verb: verb_str.to_string(),
                            feedback: match feedback_match {
                                Some(feedback) => Some(feedback.to_string()),
                                None => None,
                            },
                            important: match important_match {
                                Some(_) => true,
                                None => false,
                            },
                        }));
                    }
                    None => {
                        return Err(ParseError::InvalidFormat(
                            possible,
                            POSSIBILITY_HR.to_string(),
                        ));
                    }
                };
            } else {
                return Ok(None);
            };
        }

        if let Some(inherit) = raw.inherit.take() {
            let mut captures: Vec<Option<String>> = Vec::new();
            PARSER.service().unwrap().capture_pattern(
                POSSIBILITY_INHERIT_PATTERN.to_string(),
                inherit.trim().to_string(),
                &mut captures,
            );
            if captures.len() > 0 {
                let status_match = &captures[1];
                let verb_match = &captures[2];
                let status_str = match status_match {
                    Some(str) => str.trim().to_string(),
                    None => {
                        return Err(ParseError::InvalidFormat(
                            inherit,
                            POSSIBILITY_INHERIT_HR.to_string(),
                        ));
                    }
                };
                let verb_str = match verb_match {
                    Some(str) => str.trim().to_string(),
                    None => {
                        return Err(ParseError::InvalidFormat(
                            inherit,
                            POSSIBILITY_INHERIT_HR.to_string(),
                        ));
                    }
                };
                return Ok(Some(PossibilityParsed::Inherit {
                    status: status_str,
                    verb: verb_str,
                }));
            } else {
                return Ok(None);
            };
        }

        Err(ParseError::InvalidFormat(
            "".to_string(),
            format!(
                "({}) | ({})",
                POSSIBILITY_HR.to_string(),
                POSSIBILITY_INHERIT_HR.to_string()
            ),
        ))
    }
}
