use super::{Input, Output};

#[derive(Clone)]
pub struct Rule{
    pub chars : &'static str
}

impl From<&'static str> for Rule {
    fn from(value: &'static str) -> Self {
        return Rule{chars: value};
    }
}
impl Rule {
    pub fn fullfills(&self, input : &Input) -> Option<Output> {
        if let Some(input_char) = input.text.chars().next() {
            for char in self.chars.chars().into_iter() {
                if input_char == char {
                    return Some(1.into());
                }
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
   
}
