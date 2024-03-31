use super::{
    consequence_parsed::ConsequenceParsed, one_action_parsed::OneActionParsed,
    two_action_parsed::TwoActionParsed,
};

#[derive(Debug)]
pub enum ActionParsed {
    OneAction {
        action: OneActionParsed,
        consequences: Option<Vec<ConsequenceParsed>>,
        transition: Option<String>,
        finale: Option<String>,
    },
    TwoAction {
        action: TwoActionParsed,
        consequences: Option<Vec<ConsequenceParsed>>,
        transition: Option<String>,
        finale: Option<String>,
    },
}
