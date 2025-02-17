//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/color/rgb/one_arg/alpha.hrx"
mod alpha {
    #[allow(unused)]
    use super::rsass;
    mod clamped {
        #[allow(unused)]
        use super::rsass;
        mod alpha {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn above() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(0 0 0 / 1.1)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: black;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn below() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(0 0 0 / -0.1)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgba(0, 0, 0, 0);\
                     \n}\
                     \n"
                );
            }
        }
        #[test]
        #[ignore] // failing
        fn blue() {
            assert_eq!(
                rsass(
                    "a {b: rgb(0 0 9999 / 0.5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(0, 0, 255, 0.5);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn green() {
            assert_eq!(
                rsass(
                    "a {b: rgb(0 -1 0 / 0.5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(0, 0, 0, 0.5);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn red() {
            assert_eq!(
                rsass(
                    "a {b: rgb(256 0 0 / 0.5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(255, 0, 0, 0.5);\
                 \n}\
                 \n"
            );
        }
    }
    mod in_gamut {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: rgb($channels: 0 255 127 / 0.3)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(0, 255, 127, 0.3);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn opaque() {
            assert_eq!(
                rsass(
                    "a {b: rgb(190 173 237 / 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: #beaded;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn parenthesized() {
            assert_eq!(
        rsass(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
            \na {b: rgb(0 255 127 / 0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 255, 127, 0.3);\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // failing
        fn partial() {
            assert_eq!(
                rsass(
                    "a {b: rgb(18 52 86 / 0.5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(18, 52, 86, 0.5);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn percent() {
            assert_eq!(
                rsass(
                    "a {b: rgb(18 52 86 / 50%)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(18, 52, 86, 0.5);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn transparent() {
            assert_eq!(
                rsass(
                    "a {b: rgb(0 255 127 / 0)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: rgba(0, 255, 127, 0);\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/rgb/one_arg/no_alpha.hrx"
mod no_alpha {
    #[allow(unused)]
    use super::rsass;
    mod percents {
        #[allow(unused)]
        use super::rsass;
        mod all {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn percent() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(7.1% 20.4% 33.9%)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: #123456;\
                     \n}\
                     \n"
                );
            }
        }
        #[test]
        #[ignore] // failing
        fn boundaries() {
            assert_eq!(
                rsass(
                    "a {b: rgb(0% 100% 50%)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: #00ff80;\
                 \n}\
                 \n"
            );
        }
        mod clamped {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn blue() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(0 0 200%)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: blue;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn green() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(0 -0.1% 0)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: black;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn red() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(100.1% 0 0)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: red;\
                     \n}\
                     \n"
                );
            }
        }
        mod percent {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn green() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(190 68% 237)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: #beaded;\
                     \n}\
                     \n"
                );
            }
        }
        mod unitless {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn green() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(74.7% 173 93%)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: #beaded;\
                     \n}\
                     \n"
                );
            }
        }
    }
    mod unitless {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn beaded() {
            assert_eq!(
                rsass(
                    "a {b: rgb(190 173 237)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: #beaded;\
                 \n}\
                 \n"
            );
        }
        mod clamped {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn blue() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(0 0 9999)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: blue;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn green() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(0 -1 0)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: black;\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn red() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(256 0 0)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: red;\
                     \n}\
                     \n"
                );
            }
        }
        #[test]
        #[ignore] // failing
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: rgb($channels: 0 255 127)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: springgreen;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn numbers() {
            assert_eq!(
                rsass(
                    "a {b: rgb(18 52 86)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: #123456;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn springgreen() {
            assert_eq!(
                rsass(
                    "a {b: rgb(0 255 127)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: springgreen;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/rgb/one_arg/special_functions.hrx"
mod special_functions {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        mod calc {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(calc(1) 2 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(calc(1), 2, 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 calc(2) 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, calc(2), 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 calc(3) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 calc(3)/0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_4() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 3 / calc(0.4))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 3/calc(0.4));\
                     \n}\
                     \n"
                );
            }
        }
        mod env {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(env(--foo) 2 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(env(--foo), 2, 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 env(--foo) 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, env(--foo), 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 env(--foo) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 env(--foo)/0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_4() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 3 / env(--foo))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 3/env(--foo));\
                     \n}\
                     \n"
                );
            }
        }
        mod max {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(max(1) 2 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(max(1), 2, 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 max(2) 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, max(2), 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 max(3) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 max(3)/0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_4() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 3 / max(0.4))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 3/max(0.4));\
                     \n}\
                     \n"
                );
            }
        }
        mod min {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(min(1) 2 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(min(1), 2, 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 min(2) 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, min(2), 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 min(3) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 min(3)/0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_4() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 3 / min(0.4))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 3/min(0.4));\
                     \n}\
                     \n"
                );
            }
        }
        mod multi_argument_var {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t1_of_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(var(--foo) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(var(--foo)/0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn t1_of_2() {
                assert_eq!(
        rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: rgb(var(--foo) 2 / 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(var(--foo) 2/0.4);\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // failing
            fn t2_of_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 var(--foo) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 var(--foo)/0.4);\
                     \n}\
                     \n"
                );
            }
        }
        mod var {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(var(--foo) 2 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(var(--foo), 2, 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 var(--foo) 3 / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, var(--foo), 3, 0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 var(--foo) / 0.4)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 var(--foo)/0.4);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_4() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 3 / var(--foo))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 2 3/var(--foo));\
                     \n}\
                     \n"
                );
            }
        }
    }
    mod no_alpha {
        #[allow(unused)]
        use super::rsass;
        mod calc {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(calc(1) 2 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(calc(1), 2, 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 calc(2) 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, calc(2), 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 calc(3))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, 2, calc(3));\
                     \n}\
                     \n"
                );
            }
        }
        mod env {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(env(--foo) 2 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(env(--foo), 2, 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 env(--foo) 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, env(--foo), 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 env(--foo))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, 2, env(--foo));\
                     \n}\
                     \n"
                );
            }
        }
        mod max {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(max(1) 2 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(max(1), 2, 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 max(2) 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, max(2), 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 max(3))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, 2, max(3));\
                     \n}\
                     \n"
                );
            }
        }
        mod min {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(min(1) 2 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(min(1), 2, 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 min(2) 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, min(2), 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 min(3))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, 2, min(3));\
                     \n}\
                     \n"
                );
            }
        }
        mod multi_argument_var {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn t1_of_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(var(--foo))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(var(--foo));\
                     \n}\
                     \n"
                );
            }
            #[test]
            fn t1_of_2() {
                assert_eq!(
        rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: rgb(var(--foo) 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(var(--foo) 2);\
        \n}\
        \n"
    );
            }
            #[test]
            fn t2_of_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 var(--foo))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1 var(--foo));\
                     \n}\
                     \n"
                );
            }
        }
        mod var {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn arg_1() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(var(--foo) 2 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(var(--foo), 2, 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_2() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 var(--foo) 3)}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, var(--foo), 3);\
                     \n}\
                     \n"
                );
            }
            #[test]
            #[ignore] // failing
            fn arg_3() {
                assert_eq!(
                    rsass(
                        "a {b: rgb(1 2 var(--foo))}\
                         \n"
                    )
                    .unwrap(),
                    "a {\
                     \n  b: rgb(1, 2, var(--foo));\
                     \n}\
                     \n"
                );
            }
        }
    }
}
