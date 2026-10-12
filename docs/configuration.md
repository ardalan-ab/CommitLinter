# Configuration

`commit-linter-rs` reads `commit-linter.toml` from the current working directory. Run it from your project root so the CLI and Git hook use the same file. If the file is missing, built-in defaults are used. An invalid file stops linting with a parse error.

Create the default config:

```sh
commit-linter-rs init
```

Edit the generated `commit-linter.toml` to fit your project, then check it with a message that should pass and one that should fail:

```sh
commit-linter-rs lint --message "feat(auth): add login"
commit-linter-rs lint --message "unknown: add login"
```

If you write the file yourself, start with the complete example below. All five top-level properties are required when a file exists; individual `[rules]` switches may be omitted and default to `true`. TOML keys above `[rules]` belong at the top level, so put them before the `[rules]` heading.

When using a project-local wrapper, run the command through that ecosystem:

```sh
npx commit-linter-rs init
vendor/bin/commit-linter-rs init
dotnet tool run commit-linter-rs init
```

## Default Config

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

## Top-Level Properties

`max_header_length`

Maximum allowed length for the first line of the commit message. Used when `rules.header_max_length = true`.
The default is `100` characters.

`scope_required`

Requires commits to include a scope, such as `feat(auth): add login`. Used when `rules.scope_required = true`.
The default is `false`.

`scope_no_spaces`

Rejects scopes that contain spaces, such as `feat(auth flow): add login`. Used when `rules.scope_no_spaces = true`.
The default is `true`.

`scope_lowercase`

Requires scope values to be lowercase. Used when `rules.scope_lowercase = true`.
The default is `true`.

`types`

Allowed Conventional Commit types. Used when `rules.type_allowed = true`.
The default list is shown above. Type matching is case-sensitive.

Example:

```toml
types = ["feat", "fix", "docs", "ops"]
```

## Rule Switches

Every property under `[rules]` is a boolean. Set a rule to `false` to disable that validation.

`header_format`

Requires the header to match `type: subject` or `type(scope): subject`.

`header_max_length`

Enforces `max_header_length`.

`type_required`

Requires a type before the colon.

`type_lowercase`

Requires the type to be lowercase.

`type_allowed`

Requires the type to exist in the `types` list.

`subject_required`

Requires non-empty text after the colon.

`scope_not_empty`

Rejects empty scopes such as `fix(): bad scope`.

`scope_no_spaces`

Enforces the top-level `scope_no_spaces` setting.

`scope_lowercase`

Enforces the top-level `scope_lowercase` setting.

`scope_required`

Enforces the top-level `scope_required` setting.

## Common Examples

Keep the other required top-level properties from the default config when using these examples.

Allow custom commit types without listing them:

```toml
[rules]
type_allowed = false
```

Require scopes:

```toml
scope_required = true

[rules]
scope_required = true
```

Both settings must be `true`: the top-level setting chooses the policy, and the rule switch enables its check. The same applies to `scope_no_spaces` and `scope_lowercase`.

Allow long headers:

```toml
[rules]
header_max_length = false
```

Increase the header limit:

```toml
max_header_length = 120
```
