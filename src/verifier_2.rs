use chrono::Datelike;

type RuleVerifier = fn(String) -> Result<(), String>;

trait TimeProvider {
    fn get_day(&self) -> chrono::Weekday;
}

#[derive(Clone)]
struct RealTimeProvider;

impl TimeProvider for RealTimeProvider {
    fn get_day(&self) -> chrono::Weekday {
        chrono::Local::now().weekday()
    }
}

struct Verifier<T>
where
    T: TimeProvider,
{
    rules: Vec<RuleVerifier>,
    time_provider: T,
}

impl<T> Verifier<T>
where
    T: TimeProvider,
{
    fn new(rules: Vec<RuleVerifier>, time_provider: T) -> Self {
        Self {
            rules,
            time_provider,
        }
    }

    fn add_rule(&mut self, rule: RuleVerifier) {
        self.rules.push(rule);
    }

    fn verify(&self, password: &str) -> Vec<String> {
        if [chrono::Weekday::Sat, chrono::Weekday::Sun].contains(&self.time_provider.get_day()) {
            return vec!["It's the weekend!".into()];
        }

        let mut errors = vec![];

        for rule in self.rules.iter() {
            let result = rule(password.into());
            if let Err(error) = result {
                errors.push(error);
            }
        }

        errors
    }
}

struct VerifierFactory<T>
where
    T: TimeProvider + Clone,
{
    time_provider: T,
}

impl<T> VerifierFactory<T>
where
    T: TimeProvider + Clone,
{
    fn new(time_provider: T) -> Self {
        Self { time_provider }
    }

    fn create(&self, rules: Vec<RuleVerifier>) -> Verifier<T> {
        Verifier::new(rules, self.time_provider.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeTimeProvider {
        day: chrono::Weekday,
    }

    impl FakeTimeProvider {
        fn new(day: chrono::Weekday) -> Self {
            Self { day }
        }
    }

    impl TimeProvider for FakeTimeProvider {
        fn get_day(&self) -> chrono::Weekday {
            self.day
        }
    }

    #[test]
    fn verifier_on_weekekds_throws_error() {
        let stub_time_provider = FakeTimeProvider::new(chrono::Weekday::Sun);
        let verifier = Verifier::new(vec![], stub_time_provider);

        let errors = verifier.verify("anything");

        assert!(errors[0].contains("weekend"));
    }
}
