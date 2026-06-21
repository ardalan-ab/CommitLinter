#!/usr/bin/env node

const { spawnSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const binary = findBinary();

if (!binary) {
  console.error("commit-linter-rs binary was not found on PATH.");
  console.error("Install the Rust CLI binary first or download it from GitHub Releases.");
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), {
  stdio: "inherit",
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);

function findBinary() {
  const command = process.platform === "win32" ? "commit-linter-rs.exe" : "commit-linter-rs";
  const extensions =
    process.platform === "win32" ? (process.env.PATHEXT || ".EXE").split(";") : [""];
  const ownPath = realpath(process.argv[1]);

  for (const dir of (process.env.PATH || "").split(path.delimiter)) {
    if (!dir) continue;

    const names = process.platform === "win32" ? [command, ...extensions.map((ext) => `commit-linter-rs${ext.toLowerCase()}`)] : [command];
    for (const name of names) {
      const candidate = path.join(dir, name);
      if (realpath(candidate) && realpath(candidate) !== ownPath) {
        return candidate;
      }
    }
  }

  return null;
}

function realpath(file) {
  try {
    fs.accessSync(file, fs.constants.X_OK);
    return fs.realpathSync(file);
  } catch {
    return null;
  }
}
