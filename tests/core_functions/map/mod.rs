//! Tests auto-converted from "sass-spec/spec/core_functions/map"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/map/get.hrx"
mod get {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: map-get((1: 2, 3: 4, 5: 6), 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: map-get((1: 2, 3: 4, 5: 6), 5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 6;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: map-get((1: 2, 3: 4, 5: 6), 3)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 4;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: map-get((c: d), c)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: d;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-get($map: (c: d), $key: c)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: d;\
             \n}\
             \n"
        );
    }
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn dash_sensitive() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get((c-d: e), c_d))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: null;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get((), 1))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: null;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get((c: d), d))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: null;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/map/has_key.hrx"
mod has_key {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((1: 2, 3: 4, 5: 6), 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((1: 2, 3: 4, 5: 6), 5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((1: 2, 3: 4, 5: 6), 3)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((c: d), c)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-has-key($map: (c: d), $key: c)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((), 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: map-has-key((c: d), d)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/map/keys.hrx"
mod keys {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "$result: map-keys(());\
                 \na {\
                 \n  value: inspect($result);\
                 \n  separator: list-separator($result);\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  value: ();\
             \n  separator: comma;\
             \n}\
             \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn multiple() {
        assert_eq!(
            rsass(
                "a {b: map-keys((c: d, e: f, g: h))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: c, e, g;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-keys($map: (1: 2, 3: 4))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 1, 3;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn single() {
        assert_eq!(
            rsass(
                "$result: map-keys((1: 2));\
                 \na {\
                 \n  value: $result;\
                 \n  type: type-of($result);\
                 \n  separator: list-separator($result);\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  value: 1;\
             \n  type: list;\
             \n  separator: comma;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/map/merge.hrx"
mod merge {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn different_keys() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-merge((c: d, e: f), (1: 2, 3: 4)))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: (c: d, e: f, 1: 2, 3: 4);\
             \n}\
             \n"
        );
    }
    mod empty {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn both() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((), ()))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: ();\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((), (c: d, e: f)))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (c: d, e: f);\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-merge((c: d, e: f), ()))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (c: d, e: f);\
                 \n}\
                 \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map1", error tests are not supported yet.

            // Ignoring "map2", error tests are not supported yet.
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-merge($map1: (c: d), $map2: (1: 2)))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: (c: d, 1: 2);\
             \n}\
             \n"
        );
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        rsass(
            "a {b: inspect(map-merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d, e: 2, g: h, i: 1, j: 3);\
        \n}\
        \n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-merge((c: d, e: f), (c: 1, e: 2)))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: (c: 1, e: 2);\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/map/remove.hrx"
mod remove {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "positional_and_named", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "map", error tests are not supported yet.
        }
    }
    mod found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 1))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (3: 4, 5: 6);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 5))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (1: 2, 3: 4);\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 3))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (1: 2, 5: 6);\
                 \n}\
                 \n"
            );
        }
        mod multiple {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn all() {
                assert_eq!(
        rsass(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8, 9: 10), 1, 5, 9))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (3: 4, 7: 8);\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // failing
            fn some() {
                assert_eq!(
        rsass(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8), 1, 5, 9))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (3: 4, 7: 8);\
        \n}\
        \n"
    );
            }
        }
        #[test]
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d), c))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: ();\
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
                "a {b: inspect(map-remove($map: (c: d), $key: c))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: ();\
             \n}\
             \n"
        );
    }
    mod not_found {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((), 1))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: ();\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn multiple() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d), e, f, g))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (c: d);\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn no_keys() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d)))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (c: d);\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-remove((c: d), d))}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: (c: d);\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/map/values.hrx"
mod values {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn empty() {
        assert_eq!(
            rsass(
                "$result: map-values(());\
                 \na {\
                 \n  value: inspect($result);\
                 \n  separator: list-separator($result);\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  value: ();\
             \n  separator: comma;\
             \n}\
             \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn multiple() {
        assert_eq!(
            rsass(
                "a {b: map-values((c: d, e: f, g: h))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: d, f, h;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: map-values($map: (1: 2, 3: 4))}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2, 4;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn single() {
        assert_eq!(
            rsass(
                "$result: map-values((1: 2));\
                 \na {\
                 \n  value: $result;\
                 \n  type: type-of($result);\
                 \n  separator: list-separator($result);\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  value: 2;\
             \n  type: list;\
             \n  separator: comma;\
             \n}\
             \n"
        );
    }
}
