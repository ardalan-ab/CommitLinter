import os
import platform
import stat
import subprocess
import sys
from pathlib import Path
from typing import Optional


CONFIG_FILE = "commit-linter.toml"
HOOK_START = "# commit-linter-rs start"
HOOK_END = "# commit-linter-rs end"
LEGACY_HOOK_BLOCK = 'commit-linter-rs lint --edit "$1"'

DEFAULT_CONFIG = """max_header_length = 100
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
"""


def main() -> int:
    args = sys.argv[1:]

    if args[:1] == ["init"]:
        created = init_project(Path.cwd())
        print("Created commit-linter.toml" if created else "commit-linter.toml already exists")
        return 0

    if args[:1] == ["install"]:
        hook_status = install_hook(Path.cwd(), launcher_command(Path.cwd()))
        if hook_status == "no-git":
            print("error: could not find .git directory in the current working directory", file=sys.stderr)
            return 1

        print("Installed project-local Git commit-msg hook at .git/hooks/commit-msg")
        created = init_project(Path.cwd())
        print("Created commit-linter.toml" if created else "commit-linter.toml already exists")
        return 0

    binary = find_binary()
    if binary is None:
        print("commit-linter-rs native binary was not found in this Python package.", file=sys.stderr)
        print("Reinstall the package or use a release that includes your platform binary.", file=sys.stderr)
        return 1

    try:
        completed = subprocess.run([binary, *args])
    except FileNotFoundError:
        print("commit-linter-rs native binary was not found in this Python package.", file=sys.stderr)
        return 1

    return completed.returncode


def init_project(project_root: Path) -> bool:
    config_path = project_root / CONFIG_FILE
    if config_path.exists():
        return False

    config_path.write_text(DEFAULT_CONFIG)
    return True


def install_hook(project_root: Path, command: str) -> str:
    git_dir = project_root / ".git"
    if not git_dir.is_dir():
        return "no-git"

    hooks_dir = git_dir / "hooks"
    hooks_dir.mkdir(parents=True, exist_ok=True)

    hook_path = hooks_dir / "commit-msg"
    hook_block = f"""{HOOK_START}
PROJECT_ROOT="$(dirname "$0")/../.."
cd "$PROJECT_ROOT"
{command} lint --edit "$1"
{HOOK_END}
"""
    hook_path.write_text(merge_hook(read_text_if_exists(hook_path), hook_block))
    hook_path.chmod(hook_path.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return "installed"


def launcher_command(project_root: Path) -> str:
    local_launcher = local_python_launcher(project_root)
    if local_launcher is not None:
        return local_launcher

    executable = Path(sys.argv[0]).resolve()
    try:
        relative = executable.relative_to(project_root.resolve())
        return f"./{relative.as_posix()}"
    except ValueError:
        return "commit-linter-rs"


def local_python_launcher(project_root: Path) -> Optional[str]:
    names = ["commit-linter-rs.exe", "commit-linter-rs"] if os.name == "nt" else ["commit-linter-rs"]
    dirs = [
        project_root / ".venv" / ("Scripts" if os.name == "nt" else "bin"),
        project_root / "venv" / ("Scripts" if os.name == "nt" else "bin"),
    ]

    for directory in dirs:
        for name in names:
            candidate = directory / name
            if candidate.exists():
                return f"./{candidate.relative_to(project_root).as_posix()}"

    return None


def merge_hook(existing_hook: Optional[str], hook_block: str) -> str:
    if existing_hook is None:
        return f"#!/bin/sh\n{hook_block}"

    cleaned = remove_managed_block(existing_hook)
    cleaned = cleaned.replace(LEGACY_HOOK_BLOCK, "").rstrip()
    return f"{cleaned}\n\n{hook_block}"


def remove_managed_block(hook: str) -> str:
    start = hook.find(HOOK_START)
    end = hook.find(HOOK_END)
    if start == -1 or end == -1 or end < start:
        return hook

    return hook[:start] + hook[end + len(HOOK_END):]


def read_text_if_exists(path: Path) -> Optional[str]:
    try:
        return path.read_text()
    except FileNotFoundError:
        return None


def find_binary() -> Optional[str]:
    configured_binary = os.environ.get("COMMIT_LINTER_RS_BIN")
    if configured_binary and is_executable(Path(configured_binary)):
        return configured_binary

    bundled_binary = Path(__file__).parent / "vendor" / bundled_binary_name()
    if is_executable(bundled_binary):
        return str(bundled_binary)

    return None


def bundled_binary_name() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower()

    arch = "x64" if machine in {"x86_64", "amd64"} else machine
    if system == "darwin":
        system = "macos"
    if system == "windows":
        return f"commit-linter-rs-windows-{arch}.exe"

    return f"commit-linter-rs-{system}-{arch}"


def is_executable(path: Path) -> bool:
    return path.exists() and os.access(path, os.X_OK)


if __name__ == "__main__":
    raise SystemExit(main())
