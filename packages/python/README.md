# commit-linter-rs for Python

Project-local Python launcher for the `commit-linter-rs` Rust CLI.

```sh
python -m venv .venv
. .venv/bin/activate
pip install commit-linter-rs
commit-linter-rs install
commit-linter-rs lint --message "feat(auth): add login"
```

Run `commit-linter-rs install` from the project virtual environment. It creates `commit-linter.toml` and installs a project-local `.git/hooks/commit-msg` hook.

The package uses its bundled native binary, so no global `commit-linter-rs` command or OS-level binary install is required.

Configuration reference: `docs/configuration.md`

Local package testing: `docs/local-testing.md`
