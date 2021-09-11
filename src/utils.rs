use rand::Rng;

pub struct Engine(Vec<(&'static str, &'static str)>);

impl Engine {
    pub fn new() -> Self {
        let responses = include_str!("../responses.txt");
        let templates_and_answers = responses
            .split("\n")
            .map(|template_and_answer| template_and_answer.split_once("->").unwrap())
            .collect::<Vec<_>>();
        Self(templates_and_answers)
    }

    pub fn reply(&self, expression: &String) -> String {
        let mut answers: Vec<String> = Vec::new();
        for (template, answer) in &self.0 {
            let maybe_answer = self.match_template(&expression, &template, &answer);
            if let Some(answer) = maybe_answer {
                answers.push(answer);
            }
        }
        let default_answers = self.0.iter().filter(|(t, _)| t == &"*").collect::<Vec<_>>();
        if answers.len() > default_answers.len() {
            default_answers.iter().for_each(|(_, def_ans)| {
                if let Some(pos) = answers.iter().position(|elem| elem == def_ans) {
                    answers.swap_remove(pos);
                }
            });
        }
        let rand_idx = rand::thread_rng().gen_range(0usize..(answers.len()));
        answers[rand_idx].clone()
    }

    fn match_template(&self, expression: &String, template: &str, answer: &str) -> Option<String> {
        let mut _template: String = String::from(template);
        if _template.contains("*") {
            _template = _template.replace("*", "");

            if _template.is_empty() {
                return Some(answer.to_string())
            }
        }

        if _template.contains("<w>") {
            _template = _template.replace(" <w>", "");
            let regex = regex::Regex::new(&format!(r"((?:{}\s))(\w+)", _template)).ok()?;
            let _match = regex.find(expression)?;
            let word = _match.as_str().replace(&_template, "");
            return Some(answer.replace("<w>", &word))
        }

        if expression.contains(&_template) {
            Some(answer.to_string())
        }
        else {
            None
        }
    }

}

pub fn clean_string(string: String) -> String {
    string
        .replace(|c: char| !c.is_ascii(), "")
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn engine_finds_correct_replies() {
        let engine = Engine::new();
        assert_eq!(engine.reply(&String::from("hi")), "Hike!");
        assert_eq!(engine.reply(&String::from("She is a murderer.")), " murderer?! So cool!");
    }

    #[test]
    fn regex_is_correct() {
        let engine = Engine::new();
        let regex = regex::Regex::new(&format!(r"((?:{}\s))(\w+)", "is a")).unwrap();
        assert_eq!("is a teacher", regex.find("is a teacher").unwrap().as_str())
    }

}

