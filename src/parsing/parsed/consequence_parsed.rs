#[derive(Debug)]
pub struct ConsequenceParsed {
    pub entity: Option<String>,
    pub before: Option<String>,
    pub after: String,
    pub to: String,
    pub feedback: Option<String>,
}
