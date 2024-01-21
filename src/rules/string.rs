use super::{Input, Output};

#[derive(Clone)]
pub struct Rule{
    pub text : &'static str
}

impl From<&'static str> for Rule {
    fn from(value: &'static str) -> Self {
        return Rule{text: value};
    }
}
impl Rule {
    pub fn fullfills(&self, input : &Input) -> Option<Output> {
        if input.text.starts_with(self.text) {
            return Some(self.text.len().into());
        }
        return None;
    }

    pub fn len(&self, input: &Input) -> usize {
        return self.text.len();
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
