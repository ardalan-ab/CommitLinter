# Local Package Testing

Use this guide before publishing packages to a registry. The local packages are meant to behave like registry packages: the wrapper is installed into the project, the config file lives in that project root, and the Git hook calls the project-local wrapper.

Build the Rust binary first:

```sh
cargo build --release -p commit-linter-cli
```

Copy the binary into each wrapper package for your platform. For Linux x64:

```sh
cp target/release/commit-linter-rs packages/node/vendor/commit-linter-rs-linux-x64
cp target/release/commit-linter-rs packages/python/commit_linter_rs/vendor/commit-linter-rs-linux-x64
cp target/release/commit-linter-rs packages/php/vendor-bin/commit-linter-rs-linux-x64
cp target/release/commit-linter-rs packages/dotnet/vendor/commit-linter-rs-linux-x64
cp target/release/commit-linter-rs packages/flutter/lib/vendor/commit-linter-rs-linux-x64
chmod +x packages/node/vendor/commit-linter-rs-linux-x64
chmod +x packages/python/commit_linter_rs/vendor/commit-linter-rs-linux-x64
chmod +x packages/php/vendor-bin/commit-linter-rs-linux-x64
chmod +x packages/dotnet/vendor/commit-linter-rs-linux-x64
chmod +x packages/flutter/lib/vendor/commit-linter-rs-linux-x64
```

## Node.js

From a test project:

```sh
npm install -D /path/to/commitlinter/packages/node
```

The npm package runs setup automatically after install. It creates `commit-linter.toml` and writes `.git/hooks/commit-msg` when the project has a `.git` directory.

Manual setup:

```sh
npx commit-linter-rs install
```

## Python

Use a project virtual environment:

```sh
python -m venv .venv
. .venv/bin/activate
pip install /path/to/commitlinter/packages/python
commit-linter-rs install
```

The hook points to the project-local launcher when the command is run from the virtual environment.

## PHP Composer

Use a path repository in the test project's `composer.json`:

```json
{
  "repositories": [
    {
      "type": "path",
      "url": "/path/to/commitlinter/packages/php"
    }
  ],
  "require-dev": {
    "vendor/commit-linter-rs": "*"
  }
}
```

Then run:

```sh
composer update vendor/commit-linter-rs
vendor/bin/commit-linter-rs install
```

Composer does not run dependency lifecycle scripts the same way npm does, so run `install` once after adding the package.

## .NET

Pack the tool locally:

```sh
dotnet pack /path/to/commitlinter/packages/dotnet/CommitLinterRs.csproj -c Release -o /tmp/commit-linter-rs-nuget
```

From a test project:

```sh
dotnet new tool-manifest
dotnet tool install commit-linter-rs --add-source /tmp/commit-linter-rs-nuget
dotnet tool run commit-linter-rs install
```

The hook uses `dotnet tool run commit-linter-rs`, so it stays scoped to projects with a local tool manifest.

## Flutter / Dart

From a test Flutter project, add the local package:

```yaml
dev_dependencies:
  commit_linter_rs:
    path: /path/to/commitlinter/packages/flutter
```

Then run:

```sh
flutter pub get
dart run commit_linter_rs install
dart run commit_linter_rs lint --message "feat(auth): add login"
```

The hook invokes the project-local Dart package executable.

## Quick Checks

Valid message:

```sh
commit-linter-rs lint --message "feat(auth): add login"
```

Invalid message:

```sh
commit-linter-rs lint --message "hello world"
```
