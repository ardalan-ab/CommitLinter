pub fn print_success() {
    println!("✅ Commit message is valid");
}

pub fn print_errors(errors: &[String]) {
    println!("❌ Commit message is invalid");
    println!();
    println!("Errors:");
    println!();
    for error in errors {
        println!("* {error}");
    }
}
