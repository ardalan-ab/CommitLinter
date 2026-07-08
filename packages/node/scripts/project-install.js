const fs = require("fs");
const path = require("path");

const CONFIG_FILE = "commit-linter.toml";
const HOOK_START = "# commit-linter-rs start";
const HOOK_END = "# commit-linter-rs end";
const LEGACY_HOOK_BLOCK = `cd "$(dirname "$0")/../.."
./node_modules/.bin/commit-linter-rs lint --edit "$1"`;

const DEFAULT_CONFIG = `max_header_length = 100
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
`;

function initProject(projectRoot) {
  const configPath = path.join(projectRoot, CONFIG_FILE);
  if (fs.existsSync(configPath)) {
    return false;
  }

  fs.writeFileSync(configPath, DEFAULT_CONFIG);
  return true;
}

function installHook(projectRoot) {
  const gitDir = path.join(projectRoot, ".git");
  if (!fs.existsSync(gitDir) || !fs.statSync(gitDir).isDirectory()) {
    return "no-git";
  }

  const hooksDir = path.join(gitDir, "hooks");
  fs.mkdirSync(hooksDir, { recursive: true });

  const hookPath = path.join(hooksDir, "commit-msg");
  const hookBlock = `${HOOK_START}
PROJECT_ROOT="$(dirname "$0")/../.."
cd "$PROJECT_ROOT"
./node_modules/.bin/commit-linter-rs lint --edit "$1"
${HOOK_END}
`;
  const hook = mergeHook(readFileIfExists(hookPath), hookBlock);

  fs.writeFileSync(hookPath, hook, { mode: 0o755 });
  try {
    fs.chmodSync(hookPath, 0o755);
  } catch {
    // chmod is best-effort on platforms that do not support Unix modes.
  }

  return "installed";
}

module.exports = {
  initProject,
  installHook,
};

function readFileIfExists(file) {
  try {
    return fs.readFileSync(file, "utf8");
  } catch (error) {
    if (error && error.code === "ENOENT") {
      return null;
    }

    throw error;
  }
}

function mergeHook(existingHook, hookBlock) {
  if (!existingHook) {
    return `#!/bin/sh
${hookBlock}`;
  }

  const withoutOldBlock = existingHook
    .replace(new RegExp(`\\n?${escapeRegExp(HOOK_START)}[\\s\\S]*?${escapeRegExp(HOOK_END)}\\n?`, "m"), "\n")
    .split(LEGACY_HOOK_BLOCK)
    .join("")
    .replace(/\s+$/, "");

  return `${withoutOldBlock}

${hookBlock}`;
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
