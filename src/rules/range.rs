use super::{Input, Output};
#[derive(Clone)]
pub struct Rule{
    pub start : char,
    pub end : char
}

impl From<[char; 2]> for Rule {
    fn from(value: [char; 2]) -> Self {
        return Rule{start: value[0], end: value[1]};
    }
}
impl From<std::ops::Range<char>> for Rule {
    fn from(value: std::ops::Range<char>) -> Self {
        return Rule{start: value.start, end: value.end};
    }
}

impl Rule {
    pub fn fullfills(&self, input : &Input) -> Option<Output> {
        if let Some(c) = input.text.chars().next() {
            if c >= self.start && c <= self.end {
                return Some(1.into());
            }
        }
        return None;
    }

    pub fn len(&self, input: &Input) -> usize {
        return 1;
    }
}

#[test]
fn fullfills() {
    let rule : Rule = ['1', '7'].into();/* 
    assert_eq!(rule.fullfills("ab"), None);
    assert_eq!(rule.fullfills("0ab"), None);
    assert_eq!(rule.fullfills("1ab"), Some(1));
    assert_eq!(rule.fullfills("3ab"), Some(1));
    assert_eq!(rule.fullfills("7ab"), Some(1));
    assert_eq!(rule.fullfills("8ab"), None);*/
}