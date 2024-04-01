use crate::parsing::{
    parsed::directions_parsed::DirectionsParsed, raw::directions_raw::DirectionsRaw,
};

use super::{direction_parser::DirectionParser, parse_error::ParseError, parseable::Parseable};

pub struct DirectionsParser;

impl DirectionsParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parseable<DirectionsRaw, DirectionsParsed> for DirectionsParser {
    fn parse(&self, raw: &mut DirectionsRaw) -> Result<Option<DirectionsParsed>, ParseError> {
        let direction_parser = DirectionParser::new();

        let north = match direction_parser.parse(&mut raw.north) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let northeast = match direction_parser.parse(&mut raw.northeast) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let east = match direction_parser.parse(&mut raw.east) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let southeast = match direction_parser.parse(&mut raw.southeast) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let south = match direction_parser.parse(&mut raw.south) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let southwest = match direction_parser.parse(&mut raw.southwest) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let west = match direction_parser.parse(&mut raw.west) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let northwest = match direction_parser.parse(&mut raw.northwest) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let up = match direction_parser.parse(&mut raw.up) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        let down = match direction_parser.parse(&mut raw.down) {
            Ok(direction) => direction,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(Some(DirectionsParsed {
            north,
            northeast,
            east,
            southeast,
            south,
            southwest,
            west,
            northwest,
            up,
            down,
        }))
    }
}
