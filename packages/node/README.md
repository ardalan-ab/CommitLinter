# commit-linter-rs for Node.js

Project-local Node.js package for the `commit-linter-rs` Rust CLI.

```sh
npm install -D commit-linter-rs
npx commit-linter-rs lint --message "feat(auth): add login"
```

Installing the package creates `commit-linter.toml` in the project root and installs a project-local `.git/hooks/commit-msg` hook when the project has a `.git` directory.

The hook runs the package through `./node_modules/.bin/commit-linter-rs`, so no global `commit-linter-rs` command or OS-level binary install is required.

You can rerun setup manually:

```sh
npx commit-linter-rs install
```

Configuration reference: `docs/configuration.md`

Local package testing: `docs/local-testing.md`
