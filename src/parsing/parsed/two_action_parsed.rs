#[derive(Debug)]
pub struct TwoActionParsed {
    pub verb: String,
    pub preposition: String,
    pub supplement: String,
    pub supplement_status: String,
    pub feedback: Option<String>,
}
