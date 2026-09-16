# commit_linter_rs for Flutter and Dart

This package provides a project-local Dart launcher for the native
`commit-linter-rs` CLI. It works in Flutter projects without adding Node.js.

Add it as a development dependency:

```sh
flutter pub add --dev commit_linter_rs
```

Install the Git hook and create `commit-linter.toml`:

```sh
dart run commit_linter_rs install
```

Lint a message directly:

```sh
dart run commit_linter_rs lint --message "feat(auth): add login"
```

The installed hook runs `dart run commit_linter_rs`, keeping the linter scoped
to the Flutter project's dependencies.

The package checks `COMMIT_LINTER_RS_BIN` first, then uses the native binary
bundled for the current platform. The current MVP bundle contains Linux x64;
additional release artifacts are needed for macOS, Windows, and Linux ARM64.
