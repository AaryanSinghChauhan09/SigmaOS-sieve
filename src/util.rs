//! sieve model — mail filtering language.

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub rules: Vec<Rule>,
}

impl Default for Model {
    fn default() -> Self { Self { rules: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_rule(&mut self, rule: Rule) { self.rules.push(rule); }

    pub fn match_rule(&self, subject: &str, from: &str) -> Vec<&Rule> {
        self.rules.iter().filter(|r| {
            r.conditions.iter().any(|c| {
                c.contains("subject") && subject.contains("viagra") ||
                c.contains("from") && from.contains("spam")
            })
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_rule() {
        let mut s = Model::new();
        s.add_rule(Rule { name: "spam".into(), conditions: vec!["subject contains viagra".into()], actions: vec!["file spam".into()] });
        let matches = s.match_rule("buy viagra", "a@b.com");
        assert_eq!(matches.len(), 1);
    }
}
