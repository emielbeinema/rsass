use nom::multispace;
use num_rational::Rational;
use num_traits::One;
use parser::formalargs::call_args;
use parser::strings::{
    sass_string, sass_string_dq, sass_string_ext, sass_string_sq,
};
use parser::unit::unit;
use parser::util::{name, opt_spacelike, spacelike2};
use sass::{SassString, Value};
use std::str::from_utf8;
use value::{name_to_rgb, ListSeparator, Number, Operator};

named!(pub value_expression<&[u8], Value>,
       do_parse!(
           result: separated_nonempty_list!(
               complete!(preceded!(tag!(","), opt_spacelike)),
               space_list) >>
           trail: many0!(delimited!(opt_spacelike, tag!(","),
                                    opt_spacelike)) >>
           (if result.len() == 1 && trail.is_empty() {
               result.into_iter().next().unwrap()
           } else {
               Value::List(result, ListSeparator::Comma, false, false)
           })));

named!(pub space_list<&[u8], Value>,
       do_parse!(first: se_or_ext_string >>
                 list: fold_many0!(
                     pair!(opt!(multispace), se_or_ext_string),
                     (vec![first], false),
                     |(mut list, mut unreq): (Vec<Value>, bool), (s, item)| {
                         let mut appended = false;
                         if let (None, &Value::Literal(ref s2)) = (s, &item) {
                             if let Some(&mut Value::Literal(ref mut s1)) =
                                 list.last_mut()
                             {
                                 if s1.is_unquoted() && s2.is_unquoted() {
                                     s1.append(s2);
                                     appended = true;
                                 } else {
                                     unreq = true;
                                 }
                             } else {
                                 unreq = true;
                             }
                         }
                         if !appended {
                             list.push(item);
                         }
                         (list, unreq)
                     }) >>
                 (if list.0.len() == 1 {
                     list.0.into_iter().next().unwrap()
                 } else {
                     Value::List(list.0, ListSeparator::Space, false, list.1)
                 })));

named!(
    se_or_ext_string<Value>,
    alt_complete!(single_expression | map!(sass_string_ext, Value::Literal))
);

named!(
    single_expression<Value>,
    do_parse!(
        a: logic_expression
            >> r: fold_many0!(
                pair!(
                    delimited!(
                        opt!(multispace),
                        alt_complete!(
                            value!(Operator::And, tag!("and"))
                                | value!(Operator::Or, tag!("or"))
                        ),
                        multispace
                    ),
                    single_expression
                ),
                a,
                |a, (op, b)| Value::BinOp(
                    Box::new(a),
                    false,
                    op,
                    false,
                    Box::new(b)
                )
            )
            >> (r)
    )
);

named!(
    logic_expression<Value>,
    do_parse!(
        a: sum_expression
            >> r: fold_many0!(
                pair!(
                    delimited!(
                        opt!(multispace),
                        alt_complete!(
                            value!(Operator::Equal, tag!("=="))
                                | value!(Operator::NotEqual, tag!("!="))
                                | value!(Operator::GreaterE, tag!(">="))
                                | value!(Operator::Greater, tag!(">"))
                                | value!(Operator::LesserE, tag!("<="))
                                | value!(Operator::Lesser, tag!("<"))
                        ),
                        opt!(multispace)
                    ),
                    sum_expression
                ),
                a,
                |a, (op, b)| Value::BinOp(
                    Box::new(a),
                    false,
                    op,
                    false,
                    Box::new(b)
                )
            )
            >> (r)
    )
);

named!(
    sum_expression<Value>,
    do_parse!(
        a: term_value
            >> r: fold_many0!(
                alt_complete!(
                    do_parse!(
                        op: alt_complete!(
                            value!(Operator::Plus, tag!("+"))
                                | value!(Operator::Minus, tag!("-"))
                        ) >> opt!(spacelike2)
                            >> b: term_value
                            >> (op, b)
                    ) | do_parse!(
                        spacelike2
                            >> op: alt_complete!(
                                value!(Operator::Plus, tag!("+"))
                                    | value!(Operator::Minus, tag!("-"))
                            )
                            >> spacelike2
                            >> b: term_value
                            >> (op, b)
                    )
                ),
                a,
                |a, (op, b)| Value::BinOp(
                    Box::new(a),
                    false,
                    op,
                    false,
                    Box::new(b)
                )
            )
            >> (r)
    )
);

named!(
    term_value<Value>,
    do_parse!(
        one: single_value
            >> all: fold_many0!(
                tuple!(
                    map!(opt!(multispace), |s| s.is_some()),
                    alt_complete!(
                        value!(Operator::Multiply, tag!("*"))
                            | value!(Operator::Div, tag!("/"))
                    ),
                    map!(opt!(multispace), |s| s.is_some()),
                    map!(opt!(single_value), |v| v.unwrap_or(Value::Null))
                ),
                one,
                |a, (s1, op, s2, b)| Value::BinOp(
                    Box::new(a),
                    s1,
                    op,
                    s2,
                    Box::new(b)
                )
            )
            >> (all)
    )
);

named!(pub single_value<&[u8], Value>,
       alt_complete!(
           simple_value |
           delimited!(preceded!(tag!("("), opt_spacelike),
                      alt_complete!(
                          dictionary_inner |
                          map!(value_expression,
                               |v| Value::Paren(Box::new(v))) |
                          value!(Value::List(
                              vec![], ListSeparator::Space, false, false
                          ))
                      ),
                      terminated!(opt_spacelike, tag!(")")))
               ));

named!(
    simple_value<Value>,
    alt_complete!(
        bang
            | value!(Value::True, tag!("true"))
            | value!(Value::False, tag!("false"))
            | value!(Value::HereSelector, tag!("&"))
            | unicode_range
            | bracket_list
            | number
            | variable
            | hex_color
            | value!(Value::Null, tag!("null"))
            // Really ugly special case ... sorry.
            | value!(Value::Literal("-null".into()), tag!("-null"))
            | unary_op
            | function_call
            // And a bunch of string variants
            | map!(sass_string, literal_or_color)
            | map!(sass_string_dq, Value::Literal)
            | map!(sass_string_sq, Value::Literal)
    )
);

named!(
    bang<Value>,
    map!(
        preceded!(
            pair!(tag!("!"), opt_spacelike),
            tag!("important") // TODO Pretty much anythig goes, here?
        ),
        |s| Value::Bang(from_utf8(s).unwrap().into())
    )
);

named!(
    unicode_range<Value>,
    map!(
        recognize!(do_parse!(
            tag_no_case!("U+")
                >> a: many_m_n!(0, 6, one_of!("0123456789ABCDEFabcdef"))
                >> alt_complete!(
                    preceded!(
                        tag!("-"),
                        many_m_n!(1, 6, one_of!("0123456789ABCDEFabcdef"))
                    ) | many_m_n!(1, 6 - a.len(), one_of!("?"))
                        | value!(vec![])
                )
                >> ()
        )),
        |range| Value::UnicodeRange(from_utf8(range).unwrap().into())
    )
);

named!(
    bracket_list<Value>,
    map!(
        delimited!(tag!("["), opt!(value_expression), tag!("]")),
        |item| match item {
            Some(Value::List(list, sep, false, _)) => {
                Value::List(list, sep, true, false)
            }
            Some(single) => {
                Value::List(vec![single], ListSeparator::Space, true, false)
            }
            None => Value::List(vec![], ListSeparator::Space, true, false),
        }
    )
);

named!(
    number<Value>,
    map!(
        tuple!(
            opt!(alt!(tag!("-") | tag!("+"))),
            alt!(
                map!(
                    pair!(decimal_integer, opt!(decimal_decimals)),
                    |(n, d)| (true, if let Some(d) = d { n + d } else { n })
                ) | map!(decimal_decimals, |dec| (false, dec))
            ),
            unit
        ),
        |(sign, (lead_zero, num), unit)| Value::Numeric(
            Number {
                value: if sign == Some(b"-") { -num } else { num },
                plus_sign: sign == Some(b"+"),
                lead_zero,
            },
            unit,
        )
    )
);

named!(
    decimal_integer<Rational>,
    map!(
        fold_many1!(
            // Note: We should use bytes directly, one_of returns a char.
            one_of!("0123456789"),
            0,
            |r, d| r * 10 + isize::from(d as u8 - b'0')
        ),
        Rational::from_integer
    )
);

named!(
    decimal_decimals<Rational>,
    map!(
        preceded!(
            complete!(tag!(".")),
            fold_many1!(one_of!("0123456789"), (0, 1), |(r, n), d| (
                r * 10 + isize::from(d as u8 - b'0'),
                n * 10
            ))
        ),
        |(r, d)| Rational::new(r, d)
    )
);

named!(
    variable<Value>,
    map!(preceded!(tag!("$"), name), Value::Variable)
);

named!(
    hex_color<Value>,
    preceded!(
        tag!("#"),
        map!(
            alt!(
                tuple!(hexchar2, hexchar2, hexchar2)
                    | tuple!(hexchar, hexchar, hexchar)
            ),
            |(r, g, b)| Value::Color(
                from_hex(r),
                from_hex(g),
                from_hex(b),
                Rational::from_integer(1),
                Some(format!(
                    "#{}{}{}",
                    from_utf8(r).unwrap(),
                    from_utf8(g).unwrap(),
                    from_utf8(b).unwrap()
                )),
            )
        )
    )
);

named!(
    unary_op<Value>,
    map!(
        pair!(
            terminated!(
                alt!(
                    value!(Operator::Plus, tag!("+"))
                        | value!(Operator::Minus, tag!("-"))
                        | value!(
                            Operator::Not,
                            terminated!(tag!("not"), spacelike2)
                        )
                ),
                opt_spacelike
            ),
            single_value
        ),
        |(op, v)| Value::UnaryOp(op, Box::new(v))
    )
);

named!(
    pub function_call<Value>,
    map!(
        pair!(sass_string, call_args),
        |(name, args)| Value::Call(name, args)
    )
);

fn literal_or_color(s: SassString) -> Value {
    if let Some(val) = s.single_raw() {
        if let Some((r, g, b)) = name_to_rgb(val) {
            return Value::Color(
                r,
                g,
                b,
                Rational::one(),
                Some(val.to_string()),
            );
        }
    }
    Value::Literal(s)
}

named!(hexchar, recognize!(one_of!("0123456789ABCDEFabcdef")));

named!(hexchar2, recognize!(pair!(hexchar, hexchar)));

fn from_hex(v: &[u8]) -> Rational {
    let i = u8::from_str_radix(from_utf8(v).unwrap(), 16).unwrap() as isize;
    Rational::from_integer(if v.len() == 1 { i * 17 } else { i })
}

named!(pub dictionary<Value>,
       delimited!(preceded!(tag!("("), opt_spacelike),
                  dictionary_inner,
                  terminated!(opt_spacelike, tag!(")"))));

named!(
    dictionary_inner<Value>,
    map!(
        terminated!(
            separated_nonempty_list!(
                delimited!(opt_spacelike, tag!(","), opt_spacelike),
                pair!(
                    simple_value,
                    preceded!(
                        delimited!(opt_spacelike, tag!(":"), opt_spacelike),
                        space_list
                    )
                )
            ),
            opt!(delimited!(opt_spacelike, tag!(","), opt_spacelike))
        ),
        |items| Value::Map(items.into_iter().collect())
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    use num_rational::Rational;
    use num_traits::{One, Zero};
    use sass::CallArgs;
    use sass::Value::*;
    use value::Unit;
    use variablescope::GlobalScope;

    #[test]
    fn simple_number() {
        check_expr("4;", number(4, 1))
    }

    #[test]
    fn simple_number_neg() {
        check_expr("-4;", number(-4, 1))
    }

    #[test]
    fn simple_number_pos() {
        check_expr(
            "+4;",
            Numeric(
                Number {
                    value: Rational::new(4, 1),
                    plus_sign: true,
                    lead_zero: true,
                },
                Unit::None,
            ),
        )
    }

    #[test]
    fn simple_number_dec() {
        check_expr("4.34;", number(434, 100))
    }
    #[test]
    fn simple_number_onlydec() {
        check_expr(
            ".34;",
            Numeric(
                Number {
                    value: Rational::new(34, 100),
                    plus_sign: false,
                    lead_zero: false,
                },
                Unit::None,
            ),
        )
    }
    #[test]
    fn simple_number_onlydec_neg() {
        check_expr(
            "-.34;",
            Numeric(
                Number {
                    value: Rational::new(-34, 100),
                    plus_sign: false,
                    lead_zero: false,
                },
                Unit::None,
            ),
        )
    }
    #[test]
    fn simple_number_onlydec_pos() {
        check_expr(
            "+.34;",
            Numeric(
                Number {
                    value: Rational::new(34, 100), // actually 17/50
                    plus_sign: true,
                    lead_zero: false,
                },
                Unit::None,
            ),
        )
    }

    fn number(nom: isize, denom: isize) -> Value {
        Numeric(Number::new(Rational::new(nom, denom)), Unit::None)
    }

    #[test]
    fn simple_value_literal() {
        check_expr("rad;", Literal("rad".into()))
    }

    #[test]
    fn simple_value_literal_color() {
        check_expr(
            "red;",
            Color(
                Rational::new(255, 1),
                Rational::zero(),
                Rational::zero(),
                Rational::one(),
                Some("red".into()),
            ),
        )
    }

    #[test]
    fn simple_value_variable() {
        check_expr("$red;", Variable("red".into()))
    }

    #[test]
    fn paren_literal() {
        check_expr("(rad);", Paren(Box::new(Literal("rad".into()))))
    }

    #[test]
    fn paren_multi() {
        check_expr(
            "(rod bloe);",
            Paren(Box::new(List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Space,
                false,
                false,
            ))),
        )
    }

    #[test]
    fn paren_multi_comma() {
        check_expr(
            "(rod, bloe);",
            Paren(Box::new(List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ))),
        )
    }

    #[test]
    fn multi_comma() {
        check_expr(
            "rod, bloe;",
            List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ),
        )
    }

    #[test]
    fn paren_multi_comma_trailing() {
        check_expr(
            "(rod, bloe, );",
            Paren(Box::new(List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ))),
        )
    }

    #[test]
    fn multi_comma_trailing() {
        check_expr(
            "rod, bloe, ;",
            List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ),
        )
    }

    #[test]
    fn call_no_args() {
        check_expr("foo();", Call("foo".into(), CallArgs::default()))
    }

    #[test]
    fn call_one_arg() {
        check_expr(
            "foo(17);",
            Call(
                "foo".into(),
                CallArgs::new(vec![(None, Value::scalar(17))]),
            ),
        )
    }

    #[test]
    fn multi_expression() {
        check_expr(
            "15/10 2 3;",
            List(
                vec![
                    BinOp(
                        Box::new(Value::scalar(15)),
                        false,
                        Operator::Div,
                        false,
                        Box::new(Value::scalar(10)),
                    ),
                    Value::scalar(2),
                    Value::scalar(3),
                ],
                ListSeparator::Space,
                false,
                false,
            ),
        )
    }

    #[test]
    fn double_div() {
        check_expr(
            "15/5/3;",
            BinOp(
                Box::new(BinOp(
                    Box::new(Value::scalar(15)),
                    false,
                    Operator::Div,
                    false,
                    Box::new(Value::scalar(5)),
                )),
                false,
                Operator::Div,
                false,
                Box::new(Value::scalar(3)),
            ),
        )
    }

    #[test]
    fn color_short() {
        check_expr(
            "#AbC;",
            Color(
                Rational::new(170, 1),
                Rational::new(187, 1),
                Rational::new(204, 1),
                Rational::one(),
                Some("#AbC".into()),
            ),
        )
    }

    #[test]
    fn color_long() {
        check_expr(
            "#AaBbCc;",
            Color(
                Rational::new(170, 1),
                Rational::new(187, 1),
                Rational::new(204, 1),
                Rational::one(),
                Some("#AaBbCc".into()),
            ),
        )
    }

    #[test]
    fn parse_bracket_array() {
        check_expr(
            "[foo bar];",
            List(
                vec![Literal("foo".into()), Literal("bar".into())],
                ListSeparator::Space,
                true,
                false,
            ),
        )
    }

    #[test]
    fn parse_bracket_comma_array() {
        check_expr(
            "[foo, bar];",
            List(
                vec![Literal("foo".into()), Literal("bar".into())],
                ListSeparator::Comma,
                true,
                false,
            ),
        )
    }

    #[test]
    fn parse_bracket_empty_array() {
        check_expr("[];", List(vec![], ListSeparator::Space, true, false))
    }

    #[test]
    fn map_nq() {
        check_expr(
            "(foo: bar, baz: 17);",
            Map(vec![
                (Literal("foo".into()), Literal("bar".into())),
                (Literal("baz".into()), Value::scalar(17)),
            ].into_iter()
            .collect()),
        )
    }

    fn check_expr(expr: &str, value: Value) {
        assert_eq!(value_expression(expr.as_bytes()), Done(&b";"[..], value))
    }

    #[test]
    fn parse_extended_literal() {
        let t = value_expression_eof(b"http://#{\")\"}.com/");
        if let &Done(rest, ref result) = &t {
            assert_eq!(
                (format!("{}", result.evaluate(&GlobalScope::new())), rest),
                ("http://).com/".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }
    #[test]
    fn parse_extended_literal_in_arg() {
        let t = value_expression_eof(b"url(http://#{\")\"}.com/)");
        if let &Done(rest, ref result) = &t {
            assert_eq!(
                (format!("{}", result.evaluate(&GlobalScope::new())), rest),
                ("url(http://).com/)".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }
    #[test]
    fn parse_extended_literal_in_arg_2() {
        let t = value_expression_eof(b"url(//#{\")\"}.com/)");
        if let &Done(rest, ref result) = &t {
            assert_eq!(
                (format!("{}", result.evaluate(&GlobalScope::new())), rest),
                ("url(//).com/)".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }

    named!(
        value_expression_eof<Value>,
        terminated!(value_expression, eof!())
    );

}
