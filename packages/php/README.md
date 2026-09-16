# commit-linter-rs for PHP

Project-local Composer launcher for the `commit-linter-rs` Rust CLI.

```sh
composer require --dev ardalan-ab/commit-linter-rs
vendor/bin/commit-linter-rs install
vendor/bin/commit-linter-rs lint --message "feat(auth): add login"
```

Run `vendor/bin/commit-linter-rs install` once after adding the dev dependency. It creates `commit-linter.toml` and installs a project-local `.git/hooks/commit-msg` hook.

The hook calls `vendor/bin/commit-linter-rs`, and the package uses its bundled native binary. No global `commit-linter-rs` command or OS-level binary install is required.

Configuration reference: `docs/configuration.md`

Local package testing: `docs/local-testing.md`
