use super::{Input, Output, RulePart};
#[derive(Clone)]
pub struct Rule {
    pub rules: Vec<RulePart>,
}

impl Rule {
    pub fn fullfills(&self, input: &Input) -> Option<Output> {
        let mut total_len = 0;
        let mut subnodes = vec![];
        for rule in self.rules.iter() {
            if let Some(mut output) = rule.fullfills(&input.next(total_len)) {
                total_len = total_len + output.len;
                subnodes.append(&mut output.subnodes);
            } else {
                return None;
            }
            if input.include_silent {
                input.language.execute_silent(
                    &input.next(total_len),
                    &mut total_len,
                    &mut subnodes,
                );
            }
        }
        return Some((total_len.into(), subnodes).into());
    }

    pub fn len(&self, input: &Input) -> usize {
        return self.rules.iter().map(|rule| rule.len(input)).sum();
    }
}

impl<T: Into<RulePart>, X: IntoIterator<Item = T>> From<X> for Rule {
    fn from(value: X) -> Self {
        return Rule {
            rules: value.into_iter().map(|rule| rule.into()).collect(),
        };
    }
}

#[test]
fn fullfills() {
    let rule: Rule = ["a", "b", "cd"].into(); /*
                                              assert_eq!(rule.fullfills("abcd"), Some(4));
                                              assert_eq!(rule.fullfills("acd"), None);    */
}
