//! Sass reimplemented in rust with nom.
//!
//! Sass reimplemented in rust with nom (early stage).
//! The "r" in the name might stand for the Rust programming language,
//! for "re-implemented", or possibly for my name Rasmus.
//!
//! # Example
//!
//! ```
//! use rsass::{OutputStyle, compile_scss_file};
//!
//! let file = "tests/basic/14_imports/a.scss".as_ref();
//! let css = compile_scss_file(file, OutputStyle::Compressed).unwrap();
//!
//! assert_eq!(css, b"div span{moo:goo}\n")
//! ```
//!
//! # Sass language and implemetation status
//!
//! The sass language [is defined in its reference
//! doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
//! This implementation is incomplete but getting there, if slowly.
//!
//! Progress: ![1507](http://progressed.io/bar/150?suffix=7&scale=286)
//! of 2865 tests passed in libsass compatibility mode.
//!
//! If you want a working rust library for sass right now, you may
//! be better of with [sass-rs](https://crates.io/crates/sass-rs)
//! or [sass-alt](https://crates.io/crates/sass-alt),
//! which are rust wrappers around libsass.
//! Another alternative is [sassers](https://crates.io/crates/sassers)
//! which is another early stage pure rust implementation.
//! That said, this implementation has reached a version where I find it
//! usable for my personal projects, and the number of working tests are
//! improving.
use std::path::Path;

pub mod css;
mod error;
mod file_context;
mod functions;
mod ordermap;
mod output_style;
mod parser;
pub mod sass;
pub mod selectors;
mod value;
mod variablescope;

pub use crate::error::{ErrPos, Error};
pub use crate::file_context::FileContext;
pub use crate::functions::SassFunction;
pub use crate::output_style::OutputStyle;
pub use crate::parser::{parse_scss_data, parse_scss_file, parse_value_data};
pub use crate::sass::Item;
pub use crate::value::{set_precision, ListSeparator, Number, Quotes, Unit};
pub use crate::variablescope::{GlobalScope, Scope};
pub use num_rational::Rational;

/// Parse scss data from a buffer and write css in the given style.
///
/// # Example
///
/// ```
/// use rsass::compile_value;
///
/// assert_eq!(compile_value(b"10px + 4px").unwrap(), b"14px");
/// ```
pub fn compile_value(input: &[u8]) -> Result<Vec<u8>, Error> {
    let scope = GlobalScope::new();
    let value = parse_value_data(input)?;
    let buffer = format!("{}", value.evaluate(&scope)?).into_bytes();
    Ok(buffer)
}

/// Parse scss data from a buffer and write css in the given style.
///
/// # Example
///
/// ```
/// use rsass::{OutputStyle, compile_scss};
///
/// assert_eq!(
///     compile_scss(
///         b"foo {\
///         \n    bar {\
///         \n        baz:value;\
///         \n    }\
///         \n}",
///         OutputStyle::Compressed
///     ).unwrap(),
///     b"foo bar{baz:value}\n"
/// )
/// ```
pub fn compile_scss(
    input: &[u8],
    style: OutputStyle,
) -> Result<Vec<u8>, Error> {
    let file_context = FileContext::new();
    let items =
        parse_scss_data(input).map_err(|(pos, kind)| Error::ParseError {
            file: "-".into(),
            pos: ErrPos::pos_of(pos, input),
            kind,
        })?;
    style.write_root(&items, &mut GlobalScope::new(), &file_context)
}

/// Parse a file of scss data and write css in the given style.
///
/// Any `@import` directives will be handled relative to the directory
/// part of `file`.
///
/// # Example
///
/// ```
/// use rsass::{OutputStyle, compile_scss_file};
///
/// assert_eq!(
///     compile_scss_file(
///         "tests/basic/14_imports/a.scss".as_ref(),
///         OutputStyle::Compressed
///     ).unwrap(),
///     b"div span{moo:goo}\n"
/// )
/// ```
pub fn compile_scss_file(
    file: &Path,
    style: OutputStyle,
) -> Result<Vec<u8>, Error> {
    let file_context = FileContext::new();
    let (sub_context, file) = file_context.file(file);
    let items = parse_scss_file(&file)?;
    style.write_root(&items, &mut GlobalScope::new(), &sub_context)
}
