#[derive(Debug)]
pub enum PossibilityParsed {
    Possible {
        verb: String,
        feedback: Option<String>,
        important: bool,
    },
    Impossible {
        verb: String,
        feedback: Option<String>,
        important: bool,
    },
    Inherit {
        status: String,
        verb: String,
    },
}
