use crate::parsing::{parsed::action_parsed::ActionParsed, raw::action_raw::ActionRaw};

use super::{
    one_action_parser::{OneActionParser, ONE_ACTION_HR},
    parse_error::ParseError,
    parseable::Parseable,
    two_action_parser::{TwoActionParser, TWO_ACTION_HR},
};

pub struct ActionParser {}

impl ActionParser {
    pub fn new() -> Self {
        Self {}
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
        if let Some(action) = one_action {
            return Ok(Some(ActionParsed::OneAction {
                action,
                consequences: None,
                transition: None,
                finale: None,
            }));
        }

        let two_action_parser = TwoActionParser::new();
        let two_action = match two_action_parser.parse(&mut raw.action) {
            Ok(action) => action,
            Err(err) => {
                return Err(err);
            }
        };
        if let Some(action) = two_action {
            return Ok(Some(ActionParsed::TwoAction {
                action,
                consequences: None,
                transition: None,
                finale: None,
            }));
        }

        Err(ParseError::InvalidFormat(
            raw.action.to_string(),
            format!(
                "({}) | ({})",
                ONE_ACTION_HR.to_string(),
                TWO_ACTION_HR.to_string()
            ),
        ))
    }
}
