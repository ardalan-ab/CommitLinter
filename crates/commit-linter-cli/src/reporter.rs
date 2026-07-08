use crate::rules::LintError;

pub fn print_success() {
    println!("✅ Commit message is valid");
}

pub fn print_errors(errors: &[LintError]) {
    println!("❌ Commit message is invalid");
    println!();
    println!("Errors:");
    println!();
    for error in errors {
        println!("* [{}] {}", error.rule, error.message);
        println!("  help: {}", error.help);
    }
    println!();
    println!("Example: feat(auth): add login");
    println!("Configure rules in commit-linter.toml.");
}
