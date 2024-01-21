

use super::{Input, Output};
#[derive(Clone)]
pub struct Rule{
    pub rulename : &'static str
}

impl From<&'static str> for Rule {
    fn from(value: &'static str) -> Self {
        return Rule{rulename: value};
    }
}
impl Rule {
    pub fn fullfills(&self, input : & Input) -> Option<Output> {
        if let Some(rule) = input.language.rules.get(self.rulename) {
            let nodes = rule.fullfills(input);
            if nodes.is_empty() == false {
                let max =nodes.iter().map(|x|x.range.end).max().unwrap();
                return Some((max-input.range.start, nodes).into());
            }
        }
        return None;
    }

    pub fn len(&self, input : &Input) -> usize {
        if let Some(rule) = input.language.rules.get(self.rulename) {
            return rule.rule.len(input);
        }
        return 0;
    }
}

#[test]
fn fullfills() {
    let rule : Rule = "ab".into();/* 
    assert_eq!(rule.fullfills("ab".into()), Some(2));
    assert_eq!(rule.fullfills("abc"), Some(2));
    assert_eq!(rule.fullfills("a"), None);
    assert_eq!(rule.fullfills("ba"), None);
    assert_eq!(rule.fullfills("acbc"), None);*/
}
