use crate::parsing::{parsed::action_parsed::ActionParsed, raw::action_raw::ActionRaw};

use super::{
    consequence_parser::ConsequenceParser,
    one_action_parser::{OneActionParser, ONE_ACTION_HR},
    parse_error::ParseError,
    parseable::Parseable,
    two_action_parser::{TwoActionParser, TWO_ACTION_HR},
};

pub struct ActionParser;

impl ActionParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parseable<ActionRaw, ActionParsed> for ActionParser {
    fn parse(&self, raw: &mut ActionRaw) -> Result<Option<ActionParsed>, ParseError> {
        let one_action_parser = OneActionParser::new();
        let one_action = match one_action_parser.parse(&mut raw.action) {
            Ok(action) => action,
            Err(err) => {
                return Err(err);
            }
        };

        let two_action_parser = TwoActionParser::new();
        let two_action = match two_action_parser.parse(&mut raw.action) {
            Ok(action) => action,
            Err(err) => {
                return Err(err);
            }
        };

        if let None = one_action {
            if let None = two_action {
                return Err(ParseError::InvalidFormat(
                    raw.action.to_string(),
                    format!(
                        "`{}` | `{}`",
                        ONE_ACTION_HR.to_string(),
                        TWO_ACTION_HR.to_string()
                    ),
                ));
            }
        }

        let consequence_parser = ConsequenceParser::new();
        let mut consequences = Vec::new();
        if let Some(raw_consequences) = raw.consequences.take() {
            for mut raw_consequence in raw_consequences {
                // println!("          parsing `{}`", raw_consequence);
                match consequence_parser.parse(&mut raw_consequence) {
                    Ok(consequence_opt) => {
                        if let Some(consequence) = consequence_opt {
                            consequences.push(consequence);
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                };
            }
        }

        if let Some(action) = one_action {
            return Ok(Some(ActionParsed::OneAction {
                action,
                consequences: if consequences.len() > 0 {
                    Some(consequences)
                } else {
                    None
                },
                transition: None,
                finale: None,
            }));
        } else if let Some(action) = two_action {
            return Ok(Some(ActionParsed::TwoAction {
                action,
                consequences: if consequences.len() > 0 {
                    Some(consequences)
                } else {
                    None
                },
                transition: None,
                finale: None,
            }));
        }

        Err(ParseError::InvalidFormat(
            raw.action.to_string(),
            format!(
                "`{}` | `{}`",
                ONE_ACTION_HR.to_string(),
                TWO_ACTION_HR.to_string()
            ),
        ))
    }
}
