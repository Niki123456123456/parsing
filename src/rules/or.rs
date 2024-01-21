use super::{RulePart, Input, Output};
#[derive(Clone)]
pub struct Rule{
    pub rules : Vec<RulePart>
}

impl Rule {
    pub fn fullfills(&self, input : &Input) -> Option<Output>  {
        let mut results : Vec<_> = self.rules.iter().filter_map(|rule| rule.fullfills(input)).collect();
        results.sort_by_key(|x| std::cmp::Reverse(x.len));
        return results.into_iter().next().into();
    }

    pub fn len(&self, input: &Input) -> usize {
        todo!()
    }
}

impl<T : Into<RulePart>, X : IntoIterator<Item = T>> From<X> for Rule {
    fn from(value: X) -> Self {
        return Rule{rules: value.into_iter().map(|rule|rule.into()).collect()};
    }
}

#[test]
fn fullfills() {
    let rule : Rule = ["a", "abc", "y", "ab"].into();/* 
    assert_eq!(rule.fullfills("abcd"), Some(3));
    assert_eq!(rule.fullfills("abycd"), Some(2));
    assert_eq!(rule.fullfills("labycd"), None);*/
}