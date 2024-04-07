fn verify_password<L>(
    password: &str,
    rules: Vec<fn(String) -> Result<(), String>>,
    mut logger: L,
) -> Vec<String>
where
    L: FnMut(String),
{
    let errors = rules
        .iter()
        .filter_map(|rule| match rule(password.into()) {
            Ok(_) => None,
            Err(err) => Some(err),
        })
        .collect::<Vec<String>>();

    if errors.is_empty() {
        logger("PASSED".into())
    } else {
        logger("FAILED".into())
    }

    errors
}

trait Logger {
    fn info(&mut self, text: &str);
}

struct SimpleLogger;

impl SimpleLogger {
    fn new() -> Self {
        Self
    }
}

impl Logger for SimpleLogger {
    fn info(&mut self, text: &str) {
        println!("{text}");
    }
}

struct Verifier<L>
where
    L: Logger,
{
    rules: Vec<fn(String) -> Result<(), String>>,
    logger: L,
}

impl<L> Verifier<L>
where
    L: Logger,
{
    fn new(rules: Vec<fn(String) -> Result<(), String>>, logger: L) -> Self {
        Self { rules, logger }
    }

    fn add_rule(&mut self, rule: fn(String) -> Result<(), String>) {
        self.rules.push(rule);
    }

    fn verify(&mut self, password: &str) -> Vec<String> {
        let errors = self
            .rules
            .iter()
            .filter_map(|rule| match rule(password.into()) {
                Ok(_) => None,
                Err(err) => Some(err),
            })
            .collect::<Vec<String>>();

        if errors.is_empty() {
            self.logger.info("PASSED");
        } else {
            self.logger.info("FAILED");
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::*;

    struct FakeLogger {
        text: String,
    }

    impl FakeLogger {
        fn new() -> Self {
            Self {
                text: String::from(""),
            }
        }
    }

    impl Logger for FakeLogger {
        fn info(&mut self, text: &str) {
            self.text = text.into();
        }
    }

    #[test]
    fn verifier_with_logger_when_all_rules_pass_calls_the_logger_with_passed() {
        let mut written = String::from("");
        let mock_logger = |log| {
            written = log;
        };

        verify_password("anything", vec![], mock_logger);

        assert_eq!(written, "PASSED");
    }

    #[test]
    fn verifier_with_interfaces_verify_with_logger_calls_logger() {
        let mock_logger = FakeLogger::new();
        let mut verifier = Verifier::new(vec![], mock_logger);

        verifier.verify("anything");

        assert_eq!(verifier.logger.text, "PASSED");
    }
}
