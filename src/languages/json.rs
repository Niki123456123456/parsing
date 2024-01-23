use crate::rules::{c, ch, grammar::Grammar, n, r, rf, rule::Node};

use super::testing::test_rule;


pub fn grammar() -> Grammar {
    let mut grammar = Grammar::default();

    grammar.silent("WS", ch(" \t\n\r"));
    // exponent number
    grammar.atomic("EXP", ch("Ee") + c(ch("+-"), '?') + c(r('0'..'9'), '+'));

    // integer part forbis leading 0s (e.g. `01`)
    grammar.atomic("INT", r("0") | (r('1'..'9') + c(r('0'..'9'), '*')));

    grammar.atomic(
        "NUMBER",
        c(r("-"), '?') + rf("INT") + c(r(".") + c(r('0'..'9'), '+'), '?') + c(rf("EXP"), '?'),
    );

    grammar.atomic(
        "SAFECODEPOINT",
        n(r("\"") | r("\\") | r('\u{0000}'..'\u{001F}')),
    );

    grammar.atomic("HEX", r('0'..'9') | r('a'..'f') | r('A'..'F'));

    grammar.atomic(
        "UNICODE",
        r("u") + rf("HEX") + rf("HEX") + rf("HEX") + rf("HEX"),
    );

    grammar.atomic("ESC", r("\\") + (ch("\"\\/bfnrt") | rf("UNICODE")));

    grammar.atomic_sub(
        "STRING",
        rf("QM") + c((rf("ESC") | rf("SAFECODEPOINT")), '*') + rf("QM"),
    );

    grammar.atomic("TRUE", r("true"));
    grammar.atomic("FALSE", r("false"));
    grammar.atomic("NULL", r("null"));
    grammar.atomic("QM", r("\"")); // quotation marks
    grammar.atomic("LCBRACKET", r("{")); // left curly bracket
    grammar.atomic("RCBRACKET", r("}")); // right curly bracket
    grammar.atomic("LSBRACKET", r("[")); // left square bracket
    grammar.atomic("RSBRACKET", r("]")); // right square bracket
    grammar.atomic("COMMA", r(","));

    grammar.component("pair", rf("STRING") + r(":") + rf("value"));

    grammar.component(
        "arr",
        (rf("LSBRACKET") + rf("value") + c(rf("COMMA") + rf("value"), '*') + rf("RSBRACKET"))
            | (rf("LSBRACKET") + rf("RSBRACKET")),
    );

    grammar.component(
        "obj",
        (rf("LCBRACKET") + rf("pair") + c(rf("COMMA") + rf("pair"), '*') + rf("RCBRACKET"))
            | (rf("LCBRACKET") + rf("RCBRACKET")),
    );

    grammar.or(
        "value",
        rf("STRING") | rf("NUMBER") | rf("TRUE") | rf("FALSE") | rf("NULL") | rf("obj") | rf("arr"),
    );

    return grammar;
}

#[test]
fn value() {
    let grammar = grammar();
    test_rule(&grammar, "value", "0", &vec![Node::new("NUMBER")]);
    test_rule(&grammar, "value", " -42 ", &vec![Node::new("NUMBER")]);
    test_rule(&grammar, "value", " true ", &vec![Node::new("TRUE")]);
    test_rule(&grammar, "value", "  false", &vec![Node::new("FALSE")]);
    test_rule(&grammar, "value", "null  ", &vec![Node::new("NULL")]);
    test_rule(
        &grammar,
        "value",
        "{ }",
        &vec![Node::sub(
            "obj",
            vec![Node::new("LCBRACKET"), Node::new("RCBRACKET")],
        )],
    );
    test_rule(
        &grammar,
        "value",
        "[]",
        &vec![Node::sub(
            "arr",
            vec![Node::new("LSBRACKET"), Node::new("RSBRACKET")],
        )],
    );

    test_rule(
        &grammar,
        "value",
        "[true]",
        &vec![Node::sub(
            "arr",
            vec![
                Node::new("LSBRACKET"),
                Node::new("TRUE"),
                Node::new("RSBRACKET"),
            ],
        )],
    );
    test_rule(
        &grammar,
        "value",
        "[true,  false]",
        &vec![Node::sub(
            "arr",
            vec![
                Node::new("LSBRACKET"),
                Node::new("TRUE"),
                Node::new("COMMA"),
                Node::new("FALSE"),
                Node::new("RSBRACKET"),
            ],
        )],
    );

    test_rule(
        &grammar,
        "value",
        " \"  \\n\" ",
        &vec![Node::sub(
            "STRING",
            vec![
                Node::new("QM"),
                Node::new("SAFECODEPOINT"),
                Node::new("SAFECODEPOINT"),
                Node::new("ESC"),
                Node::new("QM"),
            ],
        )],
    );

    test_rule(
        &grammar,
        "pair",
        " \" \": 42",
        &vec![Node::sub(
            "pair",
            vec![
                Node::sub(
                    "STRING",
                    vec![Node::new("QM"), Node::new("SAFECODEPOINT"), Node::new("QM")],
                ),
                Node::new("NUMBER"),
            ],
        )],
    );

    test_rule(
        &grammar,
        "value",
        "{ \" \": 42 }",
        &vec![Node::sub(
            "obj",
            vec![
                Node::new("LCBRACKET"),
                Node::sub(
                    "pair",
                    vec![
                        Node::sub(
                            "STRING",
                            vec![Node::new("QM"), Node::new("SAFECODEPOINT"), Node::new("QM")],
                        ),
                        Node::new("NUMBER"),
                    ],
                ),
                Node::new("RCBRACKET"),
            ],
        )],
    );

    test_rule(
        &grammar,
        "value",
        "{ \" \": 42 , \n \" \": -35 }",
        &vec![Node::sub(
            "obj",
            vec![
                Node::new("LCBRACKET"),
                Node::sub(
                    "pair",
                    vec![
                        Node::sub(
                            "STRING",
                            vec![Node::new("QM"), Node::new("SAFECODEPOINT"), Node::new("QM")],
                        ),
                        Node::new("NUMBER"),
                    ],
                ),
                Node::new("COMMA"),
                Node::sub(
                    "pair",
                    vec![
                        Node::sub(
                            "STRING",
                            vec![Node::new("QM"), Node::new("SAFECODEPOINT"), Node::new("QM")],
                        ),
                        Node::new("NUMBER"),
                    ],
                ),
                Node::new("RCBRACKET"),
            ],
        )],
    );
}
