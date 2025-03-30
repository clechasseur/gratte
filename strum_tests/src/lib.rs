use gratte::{Display, EnumCount, EnumDiscriminants, EnumString};
use gratte_macros::EnumIs;

#[derive(Debug, Eq, PartialEq, EnumString, Display, EnumCount, EnumDiscriminants, EnumIs)]
pub enum Color {
    /// Docs on red
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled)]
    Green(String),
}

/// A bunch of errors
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[non_exhaustive]
#[strum_discriminants(doc = "Discriminants-only version of a bunch of errors")]
#[strum_discriminants(non_exhaustive)]
pub enum Errors {
    NotFound,
    PathError(String),
}
