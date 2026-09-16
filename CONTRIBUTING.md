# Contributing

Install stable Rust and Git, then run:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
```

Keep commit-message behavior in the Rust implementation. Ecosystem packages
should only locate and launch the same native binary. Add focused rule tests and
update configuration documentation for user-visible changes.

Use Conventional Commits for repository history.
