use super::{
    consequence_parsed::ConsequenceParsed, one_action_parsed::OneActionParsed,
    two_action_parsed::TwoActionParsed,
};

#[derive(Debug)]
pub enum PossibilityParsed {
    OneAction {
        action: OneActionParsed,
        consequences: Vec<ConsequenceParsed>,
        transition: String,
        finale: String,
    },
    TwoAction {
        action: TwoActionParsed,
        consequences: Vec<ConsequenceParsed>,
        transition: String,
        finale: String,
    },
}
