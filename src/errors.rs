use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Could not parse string to geometry")]
    ParseError,

    #[error("Could not parse a float from string")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("Could not parse an integer from string")]
    ParseIntError(#[from] std::num::ParseIntError),
}