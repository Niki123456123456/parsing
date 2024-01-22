use std::{
    ops::{Add, BitOr},
    sync::Arc,
};

use self::{grammar::Grammar, reference::Rule, rule::Node};

pub mod grammar;
pub mod rule;

pub mod arregation;
pub mod count;
pub mod not;
pub mod or;
pub mod range;
pub mod reference;
pub mod string;

pub mod chars;

#[derive(Clone)]
pub enum RulePart {
    Range(range::Rule),
    String(string::Rule),
    Chars(chars::Rule),
    Arregation(arregation::Rule),
    Or(or::Rule),
    Not(not::Rule),
    Count(count::Rule),
    Reference(reference::Rule),
}

impl From<&'static str> for RulePart {
    fn from(value: &'static str) -> Self {
        return RulePart::String(value.into());
    }
}

impl From<[char; 2]> for RulePart {
    fn from(value: [char; 2]) -> Self {
        return RulePart::Range(value.into());
    }
}

impl From<std::ops::Range<char>> for RulePart {
    fn from(value: std::ops::Range<char>) -> Self {
        return RulePart::Range(value.into());
    }
}

impl BitOr for RulePart {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        if let RulePart::Or(mut or) = self {
            or.rules.push(rhs);
            return RulePart::Or(or);
        }
        if let RulePart::Or(mut or) = rhs {
            or.rules.push(self);
            return RulePart::Or(or);
        }
        return RulePart::Or([self, rhs].into());
    }
}

impl Add for RulePart {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if let RulePart::Arregation(mut arregation) = self {
            arregation.rules.push(rhs);
            return RulePart::Arregation(arregation);
        }
        if let RulePart::Arregation(mut arregation) = rhs {
            arregation.rules.push(self);
            return RulePart::Arregation(arregation);
        }
        return RulePart::Arregation([self, rhs].into());
    }
}

pub fn rf(s: &'static str) -> RulePart {
    return RulePart::Reference(Rule { rulename: s });
}
pub fn r<T: Into<RulePart>>(s: T) -> RulePart {
    return s.into();
}

pub fn c<T: Into<RulePart>>(s: T, c: char) -> RulePart {
    return RulePart::Count(count::Rule {
        rule: Arc::new(s.into()),
        count: c.try_into().unwrap(),
    });
}

pub fn ch(s: &'static str) -> RulePart {
    return RulePart::Chars(chars::Rule { chars: s });
}

pub fn n<T: Into<RulePart>>(s: T) -> RulePart {
    return RulePart::Not(not::Rule {
        rule: Arc::new(s.into()),
    });
}

#[test]
fn add() {
    let rule: RulePart = r("a") + (r("b") + n("t")) | n("t");
}

pub struct Output {
    len: usize,
    subnodes: Vec<Node>,
}

impl From<usize> for Output {
    fn from(value: usize) -> Self {
        return Output {
            len: value,
            subnodes: vec![],
        };
    }
}
impl From<(usize, Vec<Node>)> for Output {
    fn from(value: (usize, Vec<Node>)) -> Self {
        return Output {
            len: value.0,
            subnodes: value.1,
        };
    }
}
pub struct Input<'a> {
    pub text: &'a str,
    pub include_silent: bool,
    pub language: &'a Grammar,
    pub range: std::ops::Range<usize>,
}

impl<'a> Input<'a> {
    pub fn next(&self, i: usize) -> Input<'a> {
        return Input {
            text: &self.text[i..],
            include_silent: self.include_silent,
            language: self.language,
            range: self.range.start + i..self.range.end,
        };
    }
}

impl<'a> Clone for Input<'a> {
    fn clone(&self) -> Self {
        return Input {
            text: self.text,
            include_silent: self.include_silent,
            language: self.language,
            range: self.range.clone(),
        };
    }
}

impl RulePart {
    pub fn fullfills(&self, input: &Input) -> Option<Output> {
        match self {
            RulePart::Range(rule) => rule.fullfills(input),
            RulePart::String(rule) => rule.fullfills(input),
            RulePart::Chars(rule) => rule.fullfills(input),
            RulePart::Arregation(rule) => rule.fullfills(input),
            RulePart::Or(rule) => rule.fullfills(input),
            RulePart::Not(rule) => rule.fullfills(input),
            RulePart::Count(rule) => rule.fullfills(input),
            RulePart::Reference(rule) => rule.fullfills(input),
        }
    }

    pub fn len(&self, input: &Input) -> usize {
        match self {
            RulePart::Range(rule) => rule.len(input),
            RulePart::String(rule) => rule.len(input),
            RulePart::Chars(rule) => rule.len(input),
            RulePart::Arregation(rule) => rule.len(input),
            RulePart::Or(rule) => rule.len(input),
            RulePart::Not(rule) => rule.len(input),
            RulePart::Count(rule) => rule.len(input),
            RulePart::Reference(rule) => rule.len(input),
        }
    }
}
