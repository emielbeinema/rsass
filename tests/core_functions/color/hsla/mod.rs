//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod error;

mod four_args;

// From "sass-spec/spec/core_functions/color/hsla/multi_argument_var.hrx"
mod multi_argument_var {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn t1_of_1() {
        assert_eq!(
            rsass(
                "a {b: hsla(var(--foo))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: hsla(var(--foo));\
             \n}\
             \n"
        );
    }
    #[test]
    fn t1_of_2() {
        assert_eq!(
            rsass(
                "a {b: hsla(var(--foo), 0.4)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: hsla(var(--foo), 0.4);\
             \n}\
             \n"
        );
    }
    #[test]
    fn t1_of_3() {
        assert_eq!(
        rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: hsla(var(--foo), 3%, 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: hsla(var(--foo), 3%, 0.4);\
        \n}\
        \n"
    );
    }
    #[test]
    fn t2_of_2() {
        assert_eq!(
            rsass(
                "a {b: hsla(1, var(--foo))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: hsla(1, var(--foo));\
             \n}\
             \n"
        );
    }
    #[test]
    fn t2_of_3() {
        assert_eq!(
            rsass(
                "a {b: hsla(1, var(--foo), 0.4)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: hsla(1, var(--foo), 0.4);\
             \n}\
             \n"
        );
    }
    #[test]
    fn t3_of_3() {
        assert_eq!(
            rsass(
                "a {b: hsla(1, 2%, var(--foo))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: hsla(1, 2%, var(--foo));\
             \n}\
             \n"
        );
    }
}

mod one_arg;

mod three_args;
