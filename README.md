# commit-linter-rs

`commit-linter-rs` is a universal commit message linter powered by Rust. It validates Conventional Commit headers without requiring every project to install Node.js.

Many teams like the commitlint workflow, but Python, PHP, .NET, Rust, and other non-Node projects should not need a JavaScript toolchain just to run a Git hook. This project keeps the linter engine in a native Rust CLI and provides ecosystem-friendly wrappers that delegate to that same binary.

## Status

This is an MVP. Wrappers are designed to be project-local and can bundle platform-specific native binaries. The current checked-in binary is Linux x64; publish additional platform binaries for macOS, Windows, and Linux ARM before a full multi-platform registry release.

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

The install command also creates `commit-linter.toml` if it does not already exist.

The hook runs:

```sh
commit-linter-rs lint --edit "$1"
```

## Wrappers

Node.js:

```sh
npm install -D commit-linter-rs
npx commit-linter-rs lint --message "feat(auth): add login"
```

The npm package creates `commit-linter.toml` and installs a project-local `commit-msg` hook during package installation when the project has a `.git` directory. The hook calls `./node_modules/.bin/commit-linter-rs`, so it does not depend on a global OS command.

Python:

```sh
python -m venv .venv
. .venv/bin/activate
pip install commit-linter-rs
commit-linter-rs install
commit-linter-rs lint --message "feat(auth): add login"
```

Run `install` from the project virtual environment so the hook points to the project-local launcher.

PHP Composer:

```sh
composer require --dev vendor/commit-linter-rs
vendor/bin/commit-linter-rs install
vendor/bin/commit-linter-rs lint --message "feat(auth): add login"
```

.NET:

```sh
dotnet new tool-manifest
dotnet tool install commit-linter-rs
dotnet tool run commit-linter-rs install
dotnet tool run commit-linter-rs lint --message "feat(auth): add login"
```

## Configuration

The CLI looks for `commit-linter.toml` in the current working directory. If it does not exist, defaults are used. See [docs/configuration.md](docs/configuration.md) for every property and rule.

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

[rules]
header_format = true
header_max_length = true
type_required = true
type_lowercase = true
type_allowed = true
subject_required = true
scope_not_empty = true
scope_no_spaces = true
scope_lowercase = true
scope_required = true
```

Set any rule to `false` to disable that check. For example, to allow custom commit types without editing the `types` list:

```toml
[rules]
type_allowed = false
```

## Local Package Testing

See [docs/local-testing.md](docs/local-testing.md) to test unpublished npm, Python, Composer, and .NET packages in real projects.

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

- Publish prebuilt binaries for Linux, macOS, Windows, and ARM platforms.
- Split native binaries into platform-specific packages or add automatic binary download logic.
- Support more Conventional Commit rules.
- Add JSON output for CI integrations.
- Add config discovery up the directory tree.
- Add shell completions.
