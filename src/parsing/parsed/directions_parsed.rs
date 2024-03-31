use super::direction_parsed::DirectionParsed;

#[derive(Debug)]
pub struct DirectionsParsed {
    pub north: Option<DirectionParsed>,
    pub northeast: Option<DirectionParsed>,
    pub east: Option<DirectionParsed>,
    pub southeast: Option<DirectionParsed>,
    pub south: Option<DirectionParsed>,
    pub southwest: Option<DirectionParsed>,
    pub west: Option<DirectionParsed>,
    pub northwest: Option<DirectionParsed>,
    pub up: Option<DirectionParsed>,
    pub down: Option<DirectionParsed>,
}
