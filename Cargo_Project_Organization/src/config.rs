#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub environment: String,
    pub debug: bool,
}

impl Config {
    pub fn new(environment: impl Into<String>, debug: bool) -> Self {
        Self {
            environment: environment.into(),
            debug,
        }
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_production() {
        let config = Config::new("production", false);
        assert!(config.is_production());
    }
}
