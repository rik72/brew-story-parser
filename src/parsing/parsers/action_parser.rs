use crate::parsing::{parsed::action_parsed::ActionParsed, raw::action_raw::ActionRaw};

use super::{one_action_parser::OneActionParser, parse_error::ParseError, parseable::Parseable};

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
            Ok(one_action) => one_action,
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
        // let two_action_parser = TwoActionParser::new();

        Ok(None)
    }
}
