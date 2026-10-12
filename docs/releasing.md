# First release checklist

Do not push a release tag yet. The current tag workflow publishes to npm,
PyPI, and pub.dev immediately after creating the GitHub release. Those
registries need their publisher accounts configured first. pub.dev also
requires its first version to be published interactively, so the workflow
must be adjusted to avoid trying to publish that same version again.

The automated targets are npm, PyPI, and pub.dev. Composer/Packagist and
NuGet require separate publication. Native binaries are built for Linux x64,
Linux ARM64, macOS x64, macOS ARM64, and Windows x64.

## 1. Create registry accounts and reserve names

- npm package: `commit-linter-rs`
- PyPI project: `commit-linter-rs`
- pub.dev package: `commit_linter_rs`
- Packagist package: `ardalan-ab/commit-linter-rs`
- NuGet tool: `commit-linter-rs`

Search each registry immediately before publishing. Search-engine results are
not proof that a name is available; the registry is authoritative.

## 2. Configure trusted publishing

In the GitHub repository settings, create an environment named `release` and
require approval for it if desired.

- npm: either configure npm trusted publishing for
  `ardalan-ab/CommitLinter` and `.github/workflows/release.yml`, or add a
  granular automation token as the `NPM_TOKEN` GitHub Actions secret.
- PyPI: create a pending trusted publisher for project `commit-linter-rs`,
  owner `ardalan-ab`, repository `CommitLinter`, workflow `release.yml`.
- pub.dev: the first package version normally needs to be published
  interactively. After ownership is established, enable GitHub Actions
  automated publishing for `ardalan-ab/CommitLinter` and `release.yml`.

Never commit registry tokens.

## 3. Create a Composer split repository

Packagist requires `composer.json` at the repository root. Create a dedicated
public repository, for example `ardalan-ab/commit-linter-rs-composer`, whose
root is the contents of `packages/php` from this repository. Register that
repository on Packagist as `ardalan-ab/commit-linter-rs` and enable its GitHub
hook. Each release must copy the fully populated `packages/php` directory to
that repository and create the same version tag there.

Do not register the monorepo itself on Packagist: its `composer.json` is not at
the root.

## 4. Check the version

Use one version in all of these files:

- `crates/commit-linter-cli/Cargo.toml`
- `packages/node/package.json`
- `packages/python/pyproject.toml`
- `packages/flutter/pubspec.yaml`
- `packages/dotnet/CommitLinterRs.csproj`

Composer gets its version from the Git tag. The release tag must be the same
version prefixed by `v`, for example `v0.1.0`.

## 5. Validate before publishing

Run the release workflow manually. Download the
`registry-packages` artifact and verify:

- every wrapper contains all five native binaries;
- valid messages exit with code 0 and invalid messages exit nonzero;
- `npm pack --dry-run` includes `bin`, `scripts`, `vendor`, README, and LICENSE;
- `twine check` passes for the Python wheel and source archive;
- `dart analyze` and `dart pub publish --dry-run` pass;
- Composer validates with `composer validate --strict` in the split repository;
- install each artifact in a clean temporary Git project;
- the installed `commit-msg` hook preserves any existing hook content;
- Linux x64/ARM64, macOS Intel/Apple Silicon, and Windows x64 smoke tests pass.

Also confirm the repository is clean and the release tag points to the exact
commit that passed these checks.

## 6. Publish the first version

1. Publish `packages/flutter` interactively to establish pub.dev ownership:
   `dart pub publish`. Before pushing the tag, adjust the workflow so it does
   not attempt to republish that same version.
2. Push the Composer package contents to its split repository.
3. Create and push the release tag:
   `git tag -a v0.1.0 -m "Release v0.1.0"` then `git push origin v0.1.0`.
4. Approve the protected GitHub `release` environment, if configured.
5. Tag the same version in the Composer split repository.

The tag starts `.github/workflows/release.yml`, which builds native binaries,
checks and packages the wrappers, creates a GitHub release, and attempts to
publish npm, PyPI, and pub.dev. It does not publish the .NET tool or submit
the Composer package to Packagist.

## 7. Verify after publishing

Install from each public registry in a clean project rather than from the
repository. Test `init`, `install`, one valid message, and one invalid message.
Check that the registry pages show the correct README, license, repository,
version, and supported platform information. Keep the previous version
available; registries generally should not be treated as replaceable storage.
