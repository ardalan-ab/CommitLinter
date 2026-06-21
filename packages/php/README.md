# commit-linter-rs for PHP

Composer launcher for the `commit-linter-rs` Rust CLI.

```sh
composer require --dev vendor/commit-linter-rs
vendor/bin/commit-linter-rs install
vendor/bin/commit-linter-rs lint --message "feat(auth): add login"
```

This MVP wrapper expects the Rust binary to be available on `PATH`.
