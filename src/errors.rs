use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Could not parse string to geometry")]
    ParseError,

    #[error("Attribute scaling value is 0.0")]
    FixedAttribute,

    #[error("Attribute doesn't have a name")]
    UnnamedAttribute,

    #[error("Attribute doesn't change any vertices")]
    EmptyAttribute,

    #[error("Could not parse a float from string")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("Could not parse an integer from string")]
    ParseIntError(#[from] std::num::ParseIntError),
}