# commit-linter-rs

`commit-linter-rs` is a universal commit message linter powered by Rust. It validates Conventional Commit headers without requiring every project to install Node.js.

Many teams like the commitlint workflow, but Python, PHP, .NET, Rust, and other non-Node projects should not need a JavaScript toolchain just to run a Git hook. This project keeps the linter engine in a native Rust CLI and provides ecosystem-friendly wrappers that delegate to that same binary.

## Status

This is an MVP. The wrappers expect the Rust binary to be available on `PATH`. Future versions can download platform-specific binaries automatically during package installation.

## Install The Rust CLI

Build locally:

```sh
cargo build --release -p commit-linter-cli
```

Then place `target/release/commit-linter-rs` somewhere on your `PATH`.

## Usage

Lint a direct message:

```sh
commit-linter-rs lint --message "feat(auth): add login"
```

Lint a Git commit message file:

```sh
commit-linter-rs lint --edit .git/COMMIT_EDITMSG
```

Create a default config:

```sh
commit-linter-rs init
```

Install a `commit-msg` hook:

```sh
commit-linter-rs install
```

The hook runs:

```sh
commit-linter-rs lint --edit "$1"
```

## Wrappers

Node.js:

```sh
npm install -D commit-linter-rs
npx commit-linter-rs install
npx commit-linter-rs lint --message "feat(auth): add login"
```

Python:

```sh
pip install commit-linter-rs
commit-linter-rs install
commit-linter-rs lint --message "feat(auth): add login"
```

PHP Composer:

```sh
composer require --dev vendor/commit-linter-rs
vendor/bin/commit-linter-rs install
vendor/bin/commit-linter-rs lint --message "feat(auth): add login"
```

.NET:

```sh
dotnet tool install --global CommitLinterRs
commit-linter-rs install
commit-linter-rs lint --message "feat(auth): add login"
```

## Configuration

The CLI looks for `commit-linter.toml` in the current working directory. If it does not exist, defaults are used.

```toml
max_header_length = 100
scope_required = false
scope_no_spaces = true
scope_lowercase = true

types = [
  "feat",
  "fix",
  "docs",
  "style",
  "refactor",
  "perf",
  "test",
  "build",
  "ci",
  "chore",
  "revert",
]
```

## Commit Format

Valid:

```text
feat(auth): add login flow
fix(payment): handle failed transaction
chore: update dependencies
docs(readme): update install guide
```

Invalid:

```text
Feat(auth): add login
feat(auth flow): add login
feat:
hello world
fix(): bad scope
feat(auth):
feat(auth) add login
```

## Development

Build Rust CLI:

```sh
cargo build --release -p commit-linter-cli
```

Run Rust CLI:

```sh
cargo run -p commit-linter-cli -- lint --message "feat(auth): add login"
```

Run tests:

```sh
cargo test
```

Initialize config:

```sh
commit-linter-rs init
```

Install Git hook:

```sh
commit-linter-rs install
```

## Roadmap

- Publish prebuilt binaries for Linux, macOS, and Windows.
- Add automatic binary download logic to wrappers.
- Support more Conventional Commit rules.
- Add JSON output for CI integrations.
- Add config discovery up the directory tree.
- Add shell completions.
