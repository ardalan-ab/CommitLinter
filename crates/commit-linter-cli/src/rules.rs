use crate::config::Config;
use crate::parser::CommitHeader;

pub fn validate(header: &str, parsed: Option<&CommitHeader>, config: &Config) -> Vec<String> {
    let mut errors = Vec::new();

    if header.chars().count() > config.max_header_length {
        errors.push(format!(
            "header must be at most {} characters",
            config.max_header_length
        ));
    }

    let Some(parsed) = parsed else {
        errors.push("message must match 'type: subject' or 'type(scope): subject'".into());
        return errors;
    };

    if parsed.commit_type.trim().is_empty() {
        errors.push("type is required".into());
    }

    if parsed.commit_type != parsed.commit_type.to_lowercase() {
        errors.push("type must be lowercase".into());
    }

    if !config
        .types
        .iter()
        .any(|allowed| allowed == &parsed.commit_type)
    {
        errors.push(format!("type must be one of: {}", config.types.join(", ")));
    }

    if parsed.subject.trim().is_empty() {
        errors.push("subject is required".into());
    }

    match parsed.scope.as_deref() {
        Some("") => errors.push("scope must not be empty".into()),
        Some(scope) => {
            if config.scope_no_spaces && scope.chars().any(char::is_whitespace) {
                errors.push("scope must not contain spaces".into());
            }
            if config.scope_lowercase && scope != scope.to_lowercase() {
                errors.push("scope must be lowercase".into());
            }
        }
        None if config.scope_required => errors.push("scope is required".into()),
        None => {}
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_header;

    fn lint(header: &str, config: &Config) -> Vec<String> {
        let parsed = parse_header(header);
        validate(header, parsed.as_ref(), config)
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
        assert!(errors.contains(&"type must be lowercase".to_string()));
    }

    #[test]
    fn invalid_type() {
        let errors = lint("feature(auth): add login", &Config::default());
        assert!(errors
            .iter()
            .any(|error| error.starts_with("type must be one of")));
    }

    #[test]
    fn empty_subject() {
        let errors = lint("feat:", &Config::default());
        assert!(errors
            .contains(&"message must match 'type: subject' or 'type(scope): subject'".to_string()));
    }

    #[test]
    fn scope_with_space() {
        let errors = lint("feat(auth flow): add login", &Config::default());
        assert!(errors.contains(&"scope must not contain spaces".to_string()));
    }

    #[test]
    fn empty_scope() {
        let errors = lint("fix(): bad scope", &Config::default());
        assert!(errors.contains(&"scope must not be empty".to_string()));
    }

    #[test]
    fn header_too_long() {
        let mut config = Config::default();
        config.max_header_length = 10;

        let errors = lint("fix: this header is too long", &config);
        assert!(errors.contains(&"header must be at most 10 characters".to_string()));
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
        assert!(errors.contains(&"scope is required".to_string()));
    }
}
