#!/usr/bin/env node

const { spawnSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const { initProject, installHook } = require("../scripts/project-install");

const args = process.argv.slice(2);

if (args[0] === "init") {
  const created = initProject(process.cwd());
  console.log(created ? "Created commit-linter.toml" : "commit-linter.toml already exists");
  process.exit(0);
}

if (args[0] === "install") {
  const hookStatus = installHook(process.cwd());
  if (hookStatus === "no-git") {
    console.error("error: could not find .git directory in the current working directory");
    process.exit(1);
  }

  console.log("Installed project-local Git commit-msg hook at .git/hooks/commit-msg");
  const created = initProject(process.cwd());
  console.log(created ? "Created commit-linter.toml" : "commit-linter.toml already exists");
  process.exit(0);
}

const binary = findBinary();

if (!binary) {
  console.error("commit-linter-rs native binary was not found in this npm package.");
  console.error("Reinstall the package or use a release that includes your platform binary.");
  process.exit(1);
}

const result = spawnSync(binary, args, {
  stdio: "inherit",
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);

function findBinary() {
  const configuredBinary = process.env.COMMIT_LINTER_RS_BIN;
  if (configuredBinary && isExecutable(configuredBinary)) {
    return configuredBinary;
  }

  const bundledBinary = path.join(__dirname, "..", "vendor", bundledBinaryName());
  if (isExecutable(bundledBinary)) {
    return bundledBinary;
  }

  return null;
}

function bundledBinaryName() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "linux" && arch === "x64") return "commit-linter-rs-linux-x64";
  if (platform === "linux" && arch === "arm64") return "commit-linter-rs-linux-arm64";
  if (platform === "darwin" && arch === "x64") return "commit-linter-rs-macos-x64";
  if (platform === "darwin" && arch === "arm64") return "commit-linter-rs-macos-arm64";
  if (platform === "win32" && arch === "x64") return "commit-linter-rs-windows-x64.exe";

  return `commit-linter-rs-${platform}-${arch}`;
}

function isExecutable(file) {
  return realpath(file) !== null;
}

function realpath(file) {
  try {
    fs.accessSync(file, fs.constants.X_OK);
    return fs.realpathSync(file);
  } catch {
    return null;
  }
}
