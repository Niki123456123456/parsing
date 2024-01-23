use crate::rules::{grammar::Grammar, rule::Node};

pub fn test_nachars(grammar: &Grammar, rulename: &str, chars: &str) {
    for c in chars.chars() {
        let nodes = grammar.parse(rulename, &c.to_string());
        assert_eq!(nodes.len(), 0);
    }
}

pub fn test_achars(grammar: &Grammar, rulename: &str, chars: &str) {
    for c in chars.chars() {
        let nodes = grammar.parse(rulename, &c.to_string());
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].rulename, rulename);
        assert_eq!(nodes[0].range, 0..1);
        assert_eq!(nodes[0].subnodes.len(), 0);
    }
}

pub fn test_rule(grammar: &Grammar, rulename: &str, text: &str, expected_nodes: &Vec<Node>) {
    let got_notes = grammar.parse(rulename, text);
    let equals = equals(grammar, &expected_nodes, &got_notes);
    assert!(equals);
}

pub fn equals(grammar: &Grammar, expected_nodes: &Vec<Node>, got_notes: &Vec<Node>) -> bool {
    let expected_nodes: Vec<_> = expected_nodes
        .iter()
        .filter(|x| &grammar.rules[x.rulename].silent == &false)
        .collect();
    let got_notes: Vec<_> = got_notes
        .iter()
        .filter(|x| &grammar.rules[x.rulename].silent == &false)
        .collect();
    if expected_nodes.len() != got_notes.len() {
        println!("expected: {} got: {}",  expected_nodes.len(), got_notes.len());
        return false;
    }

    for i in 0..expected_nodes.len() {
        let expected = &expected_nodes[i];
        let got = &got_notes[i];
        
        if expected.rulename != got.rulename {
            println!("expected: {} got: {}", expected.rulename, got.rulename);
            return false;
        }
        if equals(grammar, &expected.subnodes, &got.subnodes) != true {
            return false;
        }
    }
    return true;
}
