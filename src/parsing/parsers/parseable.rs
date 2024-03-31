use super::parse_error::ParseError;

pub trait Parseable<R, T> {
    fn parse(&self, raw: &mut R) -> Result<Option<T>, ParseError>;
}
