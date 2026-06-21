import subprocess
import sys
from pathlib import Path
import os
from typing import Optional


def main() -> int:
    binary = _find_binary()
    if binary is None:
        print("commit-linter-rs binary was not found on PATH.", file=sys.stderr)
        print(
            "Install the Rust CLI binary first or download it from GitHub Releases.",
            file=sys.stderr,
        )
        return 1

    try:
        completed = subprocess.run([binary, *sys.argv[1:]])
    except FileNotFoundError:
        print("commit-linter-rs binary was not found on PATH.", file=sys.stderr)
        print(
            "Install the Rust CLI binary first or download it from GitHub Releases.",
            file=sys.stderr,
        )
        return 1

    return completed.returncode


def _find_binary() -> Optional[str]:
    own_path = Path(sys.argv[0]).resolve()
    names = ["commit-linter-rs"]
    if os.name == "nt":
        names = ["commit-linter-rs.exe", "commit-linter-rs.cmd", "commit-linter-rs.bat"]

    for directory in os.environ.get("PATH", "").split(os.pathsep):
        if not directory:
            continue
        for name in names:
            candidate = Path(directory) / name
            if candidate.exists() and os.access(candidate, os.X_OK):
                resolved = candidate.resolve()
                if resolved != own_path:
                    return str(candidate)

    return None


if __name__ == "__main__":
    raise SystemExit(main())
