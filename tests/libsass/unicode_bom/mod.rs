//! Tests auto-converted from "sass-spec/spec/libsass/unicode-bom"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "utf-16-big", not expected to work yet.

// Ignoring "utf-16-little", not expected to work yet.

// From "sass-spec/spec/libsass/unicode-bom/utf-8"
#[test]
fn utf_8() {
    assert_eq!(
        rsass("\u{feff}foo { bar: baz; }").unwrap(),
        "foo {\
         \n  bar: baz;\
         \n}\
         \n"
    );
}
