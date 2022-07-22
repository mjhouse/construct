use crate::errors::Error;
use itertools::Itertools;

pub fn extract<T: std::str::FromStr>(tag: char, line: &str) -> Result<(T,T,T),Error> {
    line
        .trim_start_matches([tag,' '])
        .split_whitespace()
        .take(3)
        .map(str::parse)
        .collect::<Result<Vec<T>,_>>()
        .or(Err(Error::ParseError))?
        .into_iter()
        .collect_tuple::<(_,_,_)>()
        .ok_or(Error::ParseError)
}