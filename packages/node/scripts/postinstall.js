#!/usr/bin/env node

const { initProject, installHook } = require("./project-install");

const projectRoot = process.env.INIT_CWD;

if (!projectRoot) {
  process.exit(0);
}

try {
  const createdConfig = initProject(projectRoot);
  const hookStatus = installHook(projectRoot);

  if (createdConfig) {
    console.log("commit-linter-rs: created commit-linter.toml");
  }

  if (hookStatus === "installed") {
    console.log("commit-linter-rs: installed project-local commit-msg hook");
  } else {
    console.log("commit-linter-rs: skipped hook install because .git was not found");
  }
} catch (error) {
  console.warn(`commit-linter-rs: postinstall skipped: ${error.message}`);
}
