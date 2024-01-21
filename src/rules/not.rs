use std::sync::Arc;

use super::{RulePart, Input, Output};
#[derive(Clone)]
pub struct Rule{
    pub rule : Arc<RulePart>
}


impl Rule {
    pub fn fullfills(&self, input : &Input) -> Option<Output> {
        let result = self.rule.fullfills(input);
        if result.is_none() {
            return Some(self.rule.len(input).into());
        }
        return None;
    }

    pub fn len(&self, input : &Input) -> usize {
        return self.rule.len(input);
    }
}

#[test]
fn fullfills() {
    let rule : Rule = Rule { rule: Arc::new("a".into())  };/* 
    assert_eq!(rule.fullfills("ab"), None);
    assert_eq!(rule.fullfills("bab"), Some(1));*/
}
