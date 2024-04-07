use chrono::{Datelike, Weekday};

fn verify_password<'a>(
    password: &'a str,
    rules: Vec<fn(&'a str) -> Result<(), String>>,
) -> Vec<String> {
    let mut errors = vec![];

    for rule in rules {
        let result = rule(password);
        if let Err(error) = result {
            errors.push(error);
        }
    }

    errors
}

struct Verifier<'a> {
    rules: Vec<fn(&'a str) -> Result<(), String>>,
}

impl<'a> Verifier<'a> {
    fn new() -> Self {
        Self { rules: vec![] }
    }

    fn add_rule(&mut self, rule: fn(&'a str) -> Result<(), String>) {
        self.rules.push(rule);
    }

    fn verify(&self, password: &'a str) -> Vec<String> {
        let mut errors = vec![];

        for rule in self.rules.iter() {
            let result = rule(password);
            if let Err(error) = result {
                errors.push(error);
            }
        }

        errors
    }
}

fn upper_case_rule(password: &str) -> Result<(), String> {
    if password.to_lowercase().as_str() != password {
        return Ok(());
    }

    Err("at least one upper case needed".into())
}

fn weekend_rule(_password: &str) -> Result<(), String> {
    let day = chrono::Local::now().weekday();

    if [Weekday::Sat, Weekday::Sun].contains(&day) {
        return Err("It's the weekend!".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn get_failing_rule<'a>() -> fn(&'a str) -> Result<(), String> {
        |_p| Err("fake reason".into())
    }

    fn get_passing_rule<'a>() -> fn(&'a str) -> Result<(), String> {
        |_p| Ok(())
    }

    fn get_verifier_with_failing_rule<'a>() -> Verifier<'a> {
        let mut verifier = Verifier::new();
        verifier.add_rule(get_failing_rule());
        verifier
    }

    #[test]
    fn verify_password_given_a_failing_rule_returns_errors() {
        let password = "any value";
        let fake_rule = get_failing_rule();

        let errors = verify_password(password, vec![fake_rule]);

        assert!(errors[0].contains("fake reason"));
    }

    #[test]
    fn password_verifier_with_a_failing_rule_has_an_error_message() {
        let password = "any value";
        let verifier = get_verifier_with_failing_rule();

        let errors = verifier.verify(password);

        assert!(errors[0].contains("fake reason"));
    }

    #[test]
    fn password_verifier_with_a_failing_rule_has_exactly_one_error() {
        let password = "any value";
        let verifier = get_verifier_with_failing_rule();

        let errors = verifier.verify(password);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn password_verifier_with_failing_and_passing_rules_has_an_error_message() {
        let password = "any value";
        let mut verifier = get_verifier_with_failing_rule();
        verifier.add_rule(get_passing_rule());

        let errors = verifier.verify(password);

        assert!(errors[0].contains("fake reason"));
    }

    #[rstest]
    #[case("Hello")]
    #[case("heLlO")]
    fn upper_case_rule_with_given_one_upper_case_passes(#[case] password: &str) {
        let result = upper_case_rule(password);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn upper_case_rule_with_no_upper_case_fails() {
        let password = "hello";

        let result = upper_case_rule(password);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("one upper case needed"));
    }
}
