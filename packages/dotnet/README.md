# commit-linter-rs for .NET

.NET local tool launcher for the `commit-linter-rs` Rust CLI.

```sh
dotnet new tool-manifest
dotnet tool install commit-linter-rs
dotnet tool run commit-linter-rs install
dotnet tool run commit-linter-rs lint --message "feat(auth): add login"
```

Run `dotnet tool run commit-linter-rs install` once after adding the local tool. It creates `commit-linter.toml` and installs a project-local `.git/hooks/commit-msg` hook.

The hook calls `dotnet tool run commit-linter-rs`, and the package uses its bundled native binary. No global `commit-linter-rs` command or OS-level binary install is required.

For local wrapper testing, you can override the bundled binary:

```sh
export COMMIT_LINTER_RS_BIN=/path/to/commit-linter-rs
```

Configuration reference: `docs/configuration.md`

Local package testing: `docs/local-testing.md`
