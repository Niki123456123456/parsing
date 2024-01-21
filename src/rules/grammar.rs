use std::{collections::HashMap, usize};

use super::{
    rf,
    rule::{Node, Rule},
    Input, RulePart,
};

#[derive(Default)]
pub struct Grammar {
    pub rules: HashMap<&'static str, Rule>,
    pub silents: Option<RulePart>,
}

impl Grammar {
    pub fn parse(&self, rulename: &str, text: &str) -> Vec<Node> {
        if let Some(rule) = self.rules.get(rulename) {
            return rule.fullfills(&Input {
                text,
                include_silent: rule.include_silent,
                language: self,
                range: 0..text.len(),
            });
        }
        return vec![];
    }

    pub fn execute_silent(&self, input: &Input, len: &mut usize, sub_nodes: &mut Vec<Node>) {
        let mut i = 0;
        if let Some(silent) = &self.silents {
            while let Some(mut output) = silent.fullfills(&input.next(i)) {
                *len = *len + output.len;
                i = i + output.len;
                sub_nodes.append(&mut output.subnodes);
            }
        }
    }

    pub fn silent(&mut self, name: &'static str, rule: RulePart) {
        self.rules.insert(
            name,
            Rule {
                rule,
                include_subnodes: true,
                include_self: true,
                include_silent: false,
                silent: true,
                name,
            },
        );
        if let Some(silents) = &mut self.silents {
            *silents = silents.clone() | rf(name);
        } else {
            self.silents = Some(rf(name));
        }
    }
    pub fn atomic(&mut self, name: &'static str, rule: RulePart) {
        self.rules.insert(
            name,
            Rule {
                rule,
                include_subnodes: false,
                include_self: true,
                include_silent: false,
                silent: false,
                name,
            },
        );
    }
    pub fn atomic_sub(&mut self, name: &'static str, rule: RulePart) {
        self.rules.insert(
            name,
            Rule {
                rule,
                include_subnodes: true,
                include_self: true,
                include_silent: false,
                silent: false,
                name,
            },
        );
    }
    pub fn component(&mut self, name: &'static str, rule: RulePart) {
        self.rules.insert(
            name,
            Rule {
                rule,
                include_subnodes: true,
                include_self: true,
                include_silent: true,
                silent: false,
                name,
            },
        );
    }
    pub fn or(&mut self, name: &'static str, rule: RulePart) {
        self.rules.insert(
            name,
            Rule {
                rule,
                include_subnodes: true,
                include_self: false,
                include_silent: true,
                silent: false,
                name,
            },
        );
    }
}
