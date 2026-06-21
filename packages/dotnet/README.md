# commit-linter-rs for .NET

.NET tool launcher for the `commit-linter-rs` Rust CLI.

```sh
dotnet tool install --global CommitLinterRs
commit-linter-rs install
commit-linter-rs lint --message "feat(auth): add login"
```

This MVP wrapper expects the Rust binary to be available on `PATH`.
