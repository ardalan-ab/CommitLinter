use crate::config::Config;
use crate::parser::CommitHeader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintError {
    pub rule: &'static str,
    pub message: String,
    pub help: &'static str,
}

impl LintError {
    fn new(rule: &'static str, message: impl Into<String>, help: &'static str) -> Self {
        Self {
            rule,
            message: message.into(),
            help,
        }
    }
}

pub fn validate(header: &str, parsed: Option<&CommitHeader>, config: &Config) -> Vec<LintError> {
    let mut errors = Vec::new();
    let rules = &config.rules;

    if rules.header_max_length && header.chars().count() > config.max_header_length {
        errors.push(LintError::new(
            "header-max-length",
            format!(
                "header must be at most {} characters",
                config.max_header_length
            ),
            "Shorten the first line of the commit message.",
        ));
    }

    let Some(parsed) = parsed else {
        if rules.header_format {
            errors.push(LintError::new(
                "header-format",
                "message must match 'type: subject' or 'type(scope): subject'",
                "Use a Conventional Commit header like 'feat(auth): add login'.",
            ));
        }
        return errors;
    };

    if rules.type_required && parsed.commit_type.trim().is_empty() {
        errors.push(LintError::new(
            "type-required",
            "type is required",
            "Start the message with a type such as 'feat' or 'fix'.",
        ));
    }

    if rules.type_lowercase && parsed.commit_type != parsed.commit_type.to_lowercase() {
        errors.push(LintError::new(
            "type-lowercase",
            "type must be lowercase",
            "Use 'feat' instead of 'Feat'.",
        ));
    }

    if rules.type_allowed
        && !config
            .types
            .iter()
            .any(|allowed| allowed == &parsed.commit_type)
    {
        errors.push(LintError::new(
            "type-allowed",
            format!("type must be one of: {}", config.types.join(", ")),
            "Add the type to commit-linter.toml or use one of the allowed types.",
        ));
    }

    if rules.subject_required && parsed.subject.trim().is_empty() {
        errors.push(LintError::new(
            "subject-required",
            "subject is required",
            "Add a short description after ': '.",
        ));
    }

    match parsed.scope.as_deref() {
        Some("") if rules.scope_not_empty => errors.push(LintError::new(
            "scope-not-empty",
            "scope must not be empty",
            "Remove the empty parentheses or add a scope name.",
        )),
        Some(scope) => {
            if rules.scope_no_spaces
                && config.scope_no_spaces
                && scope.chars().any(char::is_whitespace)
            {
                errors.push(LintError::new(
                    "scope-no-spaces",
                    "scope must not contain spaces",
                    "Use a compact scope such as 'auth-flow' or 'auth'.",
                ));
            }
            if rules.scope_lowercase && config.scope_lowercase && scope != scope.to_lowercase() {
                errors.push(LintError::new(
                    "scope-lowercase",
                    "scope must be lowercase",
                    "Use 'auth' instead of 'Auth'.",
                ));
            }
        }
        None if rules.scope_required && config.scope_required => errors.push(LintError::new(
            "scope-required",
            "scope is required",
            "Add a scope, for example 'feat(auth): add login'.",
        )),
        None => {}
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_header;

    fn lint(header: &str, config: &Config) -> Vec<LintError> {
        let parsed = parse_header(header);
        validate(header, parsed.as_ref(), config)
    }

    fn has_message(errors: &[LintError], message: &str) -> bool {
        errors.iter().any(|error| error.message == message)
    }

    #[test]
    fn valid_commit_with_scope() {
        assert!(lint("feat(auth): add login flow", &Config::default()).is_empty());
    }

    #[test]
    fn valid_commit_without_scope() {
        assert!(lint("chore: update dependencies", &Config::default()).is_empty());
    }

    #[test]
    fn invalid_uppercase_type() {
        let errors = lint("Feat(auth): add login", &Config::default());
        assert!(has_message(&errors, "type must be lowercase"));
    }

    #[test]
    fn invalid_type() {
        let errors = lint("feature(auth): add login", &Config::default());
        assert!(errors
            .iter()
            .any(|error| error.message.starts_with("type must be one of")));
    }

    #[test]
    fn empty_subject() {
        let errors = lint("feat:", &Config::default());
        assert!(has_message(
            &errors,
            "message must match 'type: subject' or 'type(scope): subject'"
        ));
    }

    #[test]
    fn scope_with_space() {
        let errors = lint("feat(auth flow): add login", &Config::default());
        assert!(has_message(&errors, "scope must not contain spaces"));
    }

    #[test]
    fn empty_scope() {
        let errors = lint("fix(): bad scope", &Config::default());
        assert!(has_message(&errors, "scope must not be empty"));
    }

    #[test]
    fn header_too_long() {
        let mut config = Config::default();
        config.max_header_length = 10;

        let errors = lint("fix: this header is too long", &config);
        assert!(has_message(&errors, "header must be at most 10 characters"));
    }

    #[test]
    fn config_with_custom_type() {
        let mut config = Config::default();
        config.types.push("wip".into());

        assert!(lint("wip: sketch first pass", &config).is_empty());
    }

    #[test]
    fn scope_required_true() {
        let mut config = Config::default();
        config.scope_required = true;

        let errors = lint("feat: add login", &config);
        assert!(has_message(&errors, "scope is required"));
    }

    #[test]
    fn disabled_rule_does_not_report_error() {
        let mut config = Config::default();
        config.rules.type_allowed = false;

        assert!(lint("wip: sketch first pass", &config).is_empty());
    }
}
