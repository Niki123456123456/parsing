use super::{Input, RulePart};

pub struct Node {
    pub rulename: &'static str,
    pub range: std::ops::Range<usize>,
    pub subnodes: Vec<Node>,
}

impl Node {
    pub fn new(rulename: &'static str)->Node{
        Node { rulename, range: 0..0, subnodes: vec![] }
    }

    pub fn sub(rulename: &'static str, subnodes: Vec<Node>)->Node{
        Node { rulename, range: 0..0, subnodes }
    }
}

pub struct Rule {
    pub rule: RulePart,
    pub include_subnodes: bool,
    pub include_self: bool,
    pub silent: bool,
    pub include_silent: bool,
    pub name: &'static str,
}

impl<'a> Rule {
    pub fn fullfills(&self, input: &Input) -> Vec<Node> {
        if input.range.len() == 0 {
            return vec![];
        }
        let mut input = input.clone();
        input.include_silent = input.include_silent && self.include_silent;
        let mut total_len = 0;
        let mut subnodes = vec![];
        if input.include_silent {
            input.language.execute_silent(
                &input.next(total_len),
                &mut total_len,
                &mut subnodes,
            );
        }
        if let Some(mut output) = self.rule.fullfills(&input.next(total_len)) {
            if self.include_subnodes {
                subnodes.append(&mut output.subnodes);
            }
            if self.include_self {
                return vec![Node {
                    rulename: self.name,
                    range: input.range.start..input.range.start + output.len,
                    subnodes,
                }];
            }
            return subnodes;
        }
        return vec![];
    }
}
