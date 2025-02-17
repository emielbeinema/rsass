use super::ignore;
use super::options::Options;
use super::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;

pub struct TestFixture {
    fn_name: String,
    input: String,
    expectation: TestExpectation,
    options: Options,
}

enum TestExpectation {
    ExpectedCSS(String),
    ExpectedError(String),
}

use TestExpectation::{ExpectedCSS, ExpectedError};

impl TestFixture {
    pub fn new_ok(
        fn_name: String,
        input: String,
        expected_css: &str,
        options: Options,
    ) -> Self {
        TestFixture {
            fn_name,
            input: input,
            options: options,
            expectation: ExpectedCSS(normalize_output_css(expected_css)),
        }
    }

    pub fn new_err(
        fn_name: String,
        input: String,
        error: String,
        options: Options,
    ) -> Self {
        TestFixture {
            fn_name,
            input: input,
            expectation: ExpectedError(error),
            options: options,
        }
    }

    pub fn write_test(
        &self,
        rs: &mut dyn Write,
        precision: Option<i64>,
    ) -> Result<(), Error> {
        if let Some(ref reason) = self.options.should_skip {
            ignore(rs, &self.fn_name, reason)?;
            return Ok(());
        }
        match self.expectation {
            ExpectedError(_) => {
                // TODO: Support error tests;
                ignore(
                    rs,
                    &self.fn_name,
                    "error tests are not supported yet",
                )?;
            }
            ExpectedCSS(ref expected) => {
                rs.write_all(b"#[test]\n")?;
                if !check_test(&self.input, expected) {
                    rs.write_all(b"#[ignore] // failing\n")?;
                }
                writeln!(rs, "fn {}() {{", self.fn_name)?;
                let precision = self.options.precision.or(precision);
                if let Some(precision) = precision {
                    writeln!(rs, "    set_precision({});", precision)?;
                }
                write_test_input_expected(rs, &self.input, expected)?;
                rs.write_all(b"}\n")?;
            }
        }
        Ok(())
    }
}

/// Return a pattern function matching the 'n' in \n, unless the
/// backslash is also escaped.
fn escaped_newline() -> impl FnMut(char) -> bool {
    let mut n = 0;
    move |c: char| {
        let next_n = if c == '\\' { n + 1 } else { 0 };
        let result = n % 2 == 1 && c == 'n';
        n = next_n;
        result
    }
}

fn write_test_input_expected(
    rs: &mut dyn Write,
    input: &str,
    expected: &str,
) -> Result<(), std::io::Error> {
    let input = format!("{:?}", input)
        .replace(escaped_newline(), "\n            \\n");
    let expected =
        format!("{:?}", expected).replace(escaped_newline(), "\n        \\n");
    if input.len() + expected.len() < 45 {
        writeln!(
            rs,
            "    assert_eq!(rsass({}).unwrap(), {});",
            input, expected
        )
    } else if input.len() < 54 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({}).unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )
    } else if input.len() < 63 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({})\
             \n            .unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )
    } else {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass(\
             \n            {}\
             \n        )\
             \n        .unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )
    }
}

fn check_test(input: &str, expected_output: &str) -> bool {
    match rsass(input) {
        Ok(ref output) => output == expected_output,
        Err(_) => false,
    }
}

use rsass::{compile_scss, OutputStyle};

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| normalize_output_css(s.as_str()))
                .map_err(|e| format!("{:?}", e))
        })
}

fn normalize_output_css(css: &str) -> String {
    // Normalizes the whitespace in the given CSS to make it easier to compare. Based on:
    // https://github.com/sass/sass-spec/blob/0f59164aabb3273645fda068d0fb1b879aa3f1dc/lib/sass_spec/util.rb#L5-L7
    // NOTE: This is done on input and expected output.
    // The actual result is normalized in a simler way in the rsass function in generated tests.
    lazy_static! {
        static ref RE: Regex = Regex::new("(?:\r?\n)+").unwrap();
    }
    RE.replace_all(&css, "\n").to_string()
}
