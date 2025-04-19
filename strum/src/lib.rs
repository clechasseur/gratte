//! # `gratte` (a `strum` fork)
//!
//! `gratte` is a fork of [`strum`](https://github.com/Peternator7/strum). It defines a set of
//! macros and traits for working with enums and strings easier in Rust.
//!
//! The full version of the README can be found on [GitHub](https://github.com/clechasseur/gratte).
//!
//! ## Installing
//!
//! Add `gratte` to your dependencies:
//!
//! ```toml
//! [dependencies]
//! gratte = "1.0.1"
//! ```
//!
//! or by running:
//!
//! ```bash
//! cargo add gratte
//! ```
//!
//! ## Usage
//!
//! `gratte` has the following `derive` macros:
//!
//! | Macro               | Description                                                                                              |
//! |---------------------|----------------------------------------------------------------------------------------------------------|
//! | [EnumString]        | Converts strings to enum variants based on their name.                                                   |
//! | [Display]           | Converts enum variants to strings                                                                        |
//! | [FromRepr]          | Convert from an integer to an enum.                                                                      |
//! | [AsRefStr]          | Implement `AsRef<str>` for `MyEnum`                                                                      |
//! | [IntoStaticStr]     | Implements `From<MyEnum> for &'static str` on an enum                                                    |
//! | [EnumIter]          | Creates a new type that iterates the variants of an enum.                                                |
//! | [EnumProperty]      | Add custom properties to enum variants.                                                                  |
//! | [EnumMessage]       | Add a verbose message to enum variants.                                                                  |
//! | [EnumDiscriminants] | Generate a new type with only the discriminant names.                                                    |
//! | [EnumCount]         | Add a constant `usize` equal to the number of variants.                                                  |
//! | [VariantArray]      | Adds an associated `VARIANTS` constant which is an array of all enum discriminants                       |
//! | [VariantNames]      | Adds an associated `VARIANTS` constant which is an array of discriminant names                           |
//! | [EnumTable]         | *Experimental*, creates a new type that stores an item of a specified type for each variant of the enum. |
//!
//! [EnumString]: https://docs.rs/gratte/latest/gratte/derive.EnumString.html
//! [Display]: https://docs.rs/gratte/latest/gratte/derive.Display.html
//! [AsRefStr]: https://docs.rs/gratte/latest/gratte/derive.AsRefStr.html
//! [IntoStaticStr]: https://docs.rs/gratte/latest/gratte/derive.IntoStaticStr.html
//! [EnumIter]: https://docs.rs/gratte/latest/gratte/derive.EnumIter.html
//! [EnumIs]: https://docs.rs/gratte/latest/gratte/derive.EnumIs.html
//! [EnumProperty]: https://docs.rs/gratte/latest/gratte/derive.EnumProperty.html
//! [EnumMessage]: https://docs.rs/gratte/latest/gratte/derive.EnumMessage.html
//! [EnumDiscriminants]: https://docs.rs/gratte/latest/gratte/derive.EnumDiscriminants.html
//! [EnumCount]: https://docs.rs/gratte/latest/gratte/derive.EnumCount.html
//! [FromRepr]: https://docs.rs/gratte/latest/gratte/derive.FromRepr.html
//! [VariantArray]: https://docs.rs/gratte/latest/gratte/derive.VariantArray.html
//! [VariantNames]: https://docs.rs/gratte/latest/gratte/derive.VariantNames.html
//! [EnumTable]: https://docs.rs/gratte/latest/gratte/derive.EnumTable.html

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

// only for documentation purposes
pub mod additional_attributes;

use core::iter::FusedIterator;

#[cfg(feature = "phf")]
#[doc(hidden)]
pub use phf as _private_phf_reexport_for_macro_if_phf_feature;

/// The `ParseError` enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ParseError {
    VariantNotFound,
}

#[cfg(feature = "std")]
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // We could use our macro here, but this way we don't take a dependency on the
        // macros crate.
        match self {
            ParseError::VariantNotFound => write!(f, "Matching variant not found"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::VariantNotFound => {
                "Unable to find a variant of the given enum matching the string given. Matching \
                 can be extended with the Serialize attribute and is case sensitive."
            }
        }
    }
}

/// This trait designates that an `Enum` can be iterated over. It can
/// be auto generated using the [`EnumIter`](derive.EnumIter.html) derive macro.
///
/// # Example
///
/// ```rust
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use gratte::{EnumIter, IntoEnumIterator};
///
/// #[derive(EnumIter, Debug)]
/// enum Color {
///     Red,
///     Green { range: usize },
///     Blue(usize),
///     Yellow,
/// }
///
/// // Iterate over the items in an enum and perform some function on them.
/// fn generic_iterator<E, F>(pred: F)
/// where
///     E: IntoEnumIterator,
///     F: Fn(E),
/// {
///     for e in E::iter() {
///         pred(e)
///     }
/// }
///
/// generic_iterator::<Color, _>(|color| println!("{:?}", color));
/// ```
pub trait IntoEnumIterator: Sized {
    type Iterator: Iterator<Item = Self>
        + Clone
        + DoubleEndedIterator
        + ExactSizeIterator
        + FusedIterator;

    fn iter() -> Self::Iterator;
}

pub trait VariantIterator: Sized {
    type Iterator: Iterator<Item = Self>;

    fn iter() -> Self::Iterator;
}

pub trait VariantMetadata {
    const VARIANT_COUNT: usize;
    const VARIANT_NAMES: &'static [&'static str];

    fn variant_name(&self) -> &'static str;
}

/// Associates additional pieces of information with an Enum. This can be
/// autoimplemented by deriving `EnumMessage` and annotating your variants with
/// `#[strum(message="...")]`.
///
/// # Example
///
/// ```rust
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use gratte::EnumMessage;
///
/// #[derive(PartialEq, Eq, Debug, EnumMessage)]
/// enum Pet {
///     #[strum(message="I have a dog")]
///     #[strum(detailed_message="My dog's name is Spots")]
///     Dog,
///     /// I am documented.
///     #[strum(message="I don't have a cat")]
///     Cat,
/// }
///
/// let my_pet = Pet::Dog;
/// assert_eq!("I have a dog", my_pet.get_message().unwrap());
/// ```
pub trait EnumMessage {
    fn get_message(&self) -> Option<&'static str>;
    fn get_detailed_message(&self) -> Option<&'static str>;

    /// Get the doc comment associated with a variant if it exists.
    fn get_documentation(&self) -> Option<&'static str>;
    fn get_serializations(&self) -> &'static [&'static str];
}

/// `EnumProperty` is a trait that makes it possible to store additional information
/// with enum variants. This trait is designed to be used with the macro of the same
/// name in the `gratte_macros` crate. Currently, the string, integer and bool literals
/// are supported in attributes.
///
/// # Example
///
/// ```rust
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use gratte::EnumProperty;
///
/// #[derive(PartialEq, Eq, Debug, EnumProperty)]
/// enum Class {
///     #[strum(props(Teacher="Ms.Frizzle", Room="201", students=16, mandatory=true))]
///     History,
///     #[strum(props(Teacher="Mr.Smith"))]
///     #[strum(props(Room="103", students=10))]
///     Mathematics,
///     #[strum(props(Time="2:30", mandatory=true))]
///     Science,
/// }
///
/// let history = Class::History;
/// assert_eq!("Ms.Frizzle", history.get_str("Teacher").unwrap());
/// assert_eq!(16, history.get_int("students").unwrap());
/// assert!(history.get_bool("mandatory").unwrap());
/// ```
pub trait EnumProperty {
    fn get_str(&self, prop: &str) -> Option<&'static str>;
    fn get_int(&self, _prop: &str) -> Option<i64>;
    fn get_bool(&self, _prop: &str) -> Option<bool>;
}

/// A cheap reference-to-reference conversion. Used to convert a value to a
/// reference value with `'static` lifetime within generic code.
#[deprecated(
    since = "0.22.0",
    note = "please use `#[derive(IntoStaticStr)]` instead"
)]
pub trait AsStaticRef<T>
where
    T: ?Sized,
{
    fn as_static(&self) -> &'static T;
}

/// A trait for capturing the number of variants in Enum. This trait can be autoderived by
/// `gratte_macros`.
pub trait EnumCount {
    const COUNT: usize;
}

/// A trait for retrieving the names of each variant in Enum. This trait can
/// be autoderived by `gratte_macros`.
pub trait VariantNames {
    /// Names of the variants of this enum
    const VARIANTS: &'static [&'static str];
}

/// A trait for retrieving the enum generated by [`EnumDiscriminants`] from an associated
/// Type on the original enumeration. This trait can be autoderived by `gratte_macros`.
pub trait IntoDiscriminant {
    /// Enum listing the same variants as this enum but without any data fields
    type Discriminant;

    fn discriminant(&self) -> Self::Discriminant;
}

/// A trait for retrieving a static array containing all the variants in an Enum.
/// This trait can be autoderived by `gratte_macros`. For derived usage, all the
/// variants in the enumerator need to be unit-types, which means you can't autoderive
/// enums with inner data in one or more variants. Consider using it alongside
/// [`EnumDiscriminants`] if you require inner data but still want to have an
/// static array of variants.
pub trait VariantArray: ::core::marker::Sized + 'static {
    const VARIANTS: &'static [Self];
}

#[cfg(feature = "derive")]
pub use gratte_macros::*;

macro_rules! DocumentMacroRexports {
    ($($export:ident),+) => {
        $(
            #[cfg(all(docsrs, feature = "derive"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
            pub use gratte_macros::$export;
        )+
    };
}

// We actually only re-export these items individually if we're building
// for docsrs. You can do a weird thing where you rename the macro
// and then reference it through gratte. The renaming feature should be deprecated now that
// 2018 edition is almost 2 years old, but we'll need to give people some time to do that.
DocumentMacroRexports! {
    AsRefStr,
    Display,
    EnumCount,
    EnumDiscriminants,
    EnumIter,
    EnumMessage,
    EnumProperty,
    EnumString,
    VariantNames,
    FromRepr,
    IntoStaticStr,
    VariantArray
}
