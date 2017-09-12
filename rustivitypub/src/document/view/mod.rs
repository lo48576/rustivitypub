//! Object views.

use document::JsonValue;
pub use self::document::DocumentView;
pub use self::error::{Result, PropertyError};
pub use self::image::ImageView;
pub use self::link::LinkView;
pub use self::object::ObjectView;
pub use self::object_or_link::{ObjectOrLinkView, ImageOrLinkView};
pub use self::single_or_multi::{SingleOrMultiJsonView, SingleOrMultiJsonViewIter};
pub use self::single_or_multi::{SingleOrMultiView, SingleOrMultiViewIter};
pub use self::value::{NaturalLanguageView, LangStringView, IriView, DateTimeView};
pub use self::value::{MediaTypeView, DurationView, LanguageTagView};

pub mod document;
pub mod error;
pub mod fetch;
pub mod image;
pub mod link;
pub mod object;
pub mod object_or_link;
pub mod single_or_multi;
pub mod value;


/// Attempt to construct `Self` via a conversion.
///
/// NOTE: Use
/// [`std::convert::TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html)
/// when it is stabilized.
pub trait TryFromJsonValue<'a>: Sized {
    /// Performs the conversion.
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self>;

    /// Checks if the `Self` type object can be created from the given JSON value.
    ///
    /// Returns `Err(_)` if the given JSON value cannot be converted into `Self`.
    /// Note that returned `Ok(())` doesn't always means the type conversion would succeed.
    ///
    /// This should be called at the head of `try_from_json_value()`.
    fn validate_json_value(_value: &JsonValue) -> Result<()> {
        Ok(())
    }
}

impl<'a> TryFromJsonValue<'a> for &'a str {
    fn try_from_json_value(value: &'a JsonValue) -> Result<Self> {
        Self::validate_json_value(value)?;
        match *value {
            JsonValue::String(ref s) => Ok(s),
            ref v => unreachable!("`validate_json_value()` should deny `{:?}`", v),
        }
    }

    fn validate_json_value(value: &JsonValue) -> Result<()> {
        match *value {
            JsonValue::String(_) => Ok(()),
            _ => Err(PropertyError::TypeMismatch),
        }
    }
}
