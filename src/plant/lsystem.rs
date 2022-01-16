use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LElement {
    pub alphabet: char,
    pub age: i32,
}

impl From<char> for LElement {
    fn from(alphabet: char) -> Self {
        Self {
            alphabet,
            age: 0,
        }
    }
}

impl LElement {
    fn grown(mut self) -> Self {
        self.age += 1;
        self
    }
}

pub type LRules = HashMap<char, Vec<char>>;

/// Increamental Slow LSystem. Rules are only applied on the leaf alphabets(end of string or brackets)
pub struct ILSystem {
    axiom: String,
    rules: LRules,
    production: Vec<LElement>,
    iteration: usize,
}

impl ILSystem {
    pub fn new(axiom: String, rules: LRules) -> Self {
        Self {
            production: axiom.chars().map(LElement::from).collect(),
            iteration: 0,
            axiom,
            rules,
        }
    }
    pub fn production(&self) -> &[LElement] {
        self.production.as_slice()
    }
    pub fn next(&mut self) -> &[LElement] {
        let mut new_production: Vec<LElement> = Vec::with_capacity(self.production.len() * 2);
        for (i, e) in self.production.iter().enumerate() {
            new_production.push((*e).grown());
            if i == self.production.len() - 1 || self.production[i + 1].alphabet == ']' {
                if let Some(rule) = self.rules.get(&e.alphabet) {
                    for r in rule.iter() {
                        new_production.push((*r).into());
                    }
                }
            }
        }
        self.production = new_production;
        self.iteration += 1;
        self.production.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_iterates_increamentaly() {
        let mut ilsystem = ILSystem::new("F[+FF]F".to_string(), [
            ('F', vec!['A', 'B'])
        ].into_iter().collect());
        let production = ilsystem.next();
        assert_eq!(production, &[
            LElement { alphabet: 'F', age: 1 }, 
            LElement { alphabet: '[', age: 1 }, 
            LElement { alphabet: '+', age: 1 }, 
            LElement { alphabet: 'F', age: 1 }, 
            LElement { alphabet: 'F', age: 1 }, 
            LElement { alphabet: 'A', age: 0 }, 
            LElement { alphabet: 'B', age: 0 }, 
            LElement { alphabet: ']', age: 1 }, 
            LElement { alphabet: 'F', age: 1 },
            LElement { alphabet: 'A', age: 0 },
            LElement { alphabet: 'B', age: 0 }
        ]);
    }
}
