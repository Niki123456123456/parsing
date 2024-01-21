use std::sync::Arc;

use super::{RulePart, Input, Output};
#[derive(Clone)]
pub struct Rule {
    pub rule: Arc<RulePart>,
    pub count: Count,
}
#[derive(Clone)]
pub enum Count {
    ZeroOrOnce,
    OnceOrMore,
    ZeroOrMore,
}

impl TryFrom<char> for Count {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Count::ZeroOrOnce),
            '+' => Ok(Count::OnceOrMore),
            '*' => Ok(Count::ZeroOrMore),
            _ => Err("char must be ? + or *"),
        }
    }
}

fn fullfills(rule: &RulePart, input : &Input, min: Option<usize>, max: Option<usize>) -> Option<Output> {
    let mut total_len = 0;
    let mut count: usize = 0;
    let mut subnodes = vec![];
    while let Some(mut output) = rule.fullfills(&input.next(total_len)) {
        total_len = total_len + output.len;
        count = count + 1;
        subnodes.append(&mut output.subnodes);
        if Some(count) == max {
            return Some((total_len.into(), subnodes).into());
        }
        if input.include_silent {
            input.language.execute_silent(&input.next(total_len), &mut total_len, &mut subnodes);
        }
    }
    if Some(count) < min {
        return None;
    }
    
    return Some((total_len.into(), subnodes).into());
}

impl Rule {
    pub fn fullfills(&self, input : &Input) -> Option<Output> {
        match self.count {
            Count::ZeroOrOnce => fullfills(&self.rule, input, None, Some(1)),
            Count::OnceOrMore => fullfills(&self.rule, input, Some(1), None),
            Count::ZeroOrMore => fullfills(&self.rule, input, None, None),
        }
    }

    pub fn len(&self, input: &Input) -> usize {
        todo!()
    }
}
