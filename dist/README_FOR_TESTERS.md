# commit-linter-rs local test packages

These files are local test artifacts, not registry releases yet.

## Rust binary

Use `dist/bin/commit-linter-rs-linux-x64` as the real linter binary on Linux x64. Put it on PATH as `commit-linter-rs`, or set wrapper-specific env vars where supported.

```sh
chmod +x dist/bin/commit-linter-rs-linux-x64
sudo cp dist/bin/commit-linter-rs-linux-x64 /usr/local/bin/commit-linter-rs
```

## npm

```sh
npm install ./dist/npm/commit-linter-rs-0.1.0.tgz
npx commit-linter-rs lint --message "feat(test): check npm package"
```

## pip

```sh
pip install ./dist/python/commit_linter_rs-0.1.0-py3-none-any.whl
commit-linter-rs lint --message "feat(test): check pip package"
```

## Composer

Use the archive in `dist/composer`, or send `packages/php` as a path repository during local tests. Composer must be installed on the tester machine.

## NuGet / dotnet tool

A real `.nupkg` requires the .NET SDK. If `dist/nuget` is empty, build it on a machine with dotnet installed:

```sh
dotnet pack packages/dotnet/CommitLinterRs.csproj -c Release -o dist/nuget
dotnet tool install --global commit-linter-rs --add-source dist/nuget
export COMMIT_LINTER_RS_BIN=/path/to/real/commit-linter-rs
commit-linter-rs lint --message "feat(test): check dotnet package"
```
