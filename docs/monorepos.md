# Mixed-stack monorepos

Git has one `commit-msg` hook per repository. Install one `commit-linter-rs` launcher at the repository root and keep one `commit-linter.toml` there. The hook checks the commit header for the whole repository, regardless of whether a change came from a Node, Python, PHP, .NET, Dart, or Rust package.

If the root project uses Node.js, run this from the directory containing `.git`:

```sh
npm install -D commit-linter-rs
npx commit-linter-rs install
```

If the root project uses Python instead, run:

```sh
python -m venv .venv
. .venv/bin/activate
pip install commit-linter-rs
commit-linter-rs install
```

Edit the root `commit-linter.toml` using the [configuration guide](configuration.md), then test a commit header from the root:

```sh
npx commit-linter-rs lint --message "feat(api): add endpoint"
```

For the Python installation, replace `npx commit-linter-rs` with `.venv/bin/commit-linter-rs` (or `.venv/Scripts/commit-linter-rs.exe` on Windows).

The current wrappers look for `.git` and `commit-linter.toml` in their current directory. Installing inside a workspace subdirectory does not install the repository hook. The linter does not select a different config for each package or lint based on changed paths. For package-specific rules, use separate repositories or a custom root hook that runs the relevant checks.
