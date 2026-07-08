using System.Diagnostics;
using System.Runtime.InteropServices;

const string ConfigFile = "commit-linter.toml";
const string HookStart = "# commit-linter-rs start";
const string HookEnd = "# commit-linter-rs end";
const string LegacyHookBlock = "commit-linter-rs lint --edit \"$1\"";
const string DefaultConfig = """
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
""";

if (args is ["init", ..])
{
    var created = InitProject(Directory.GetCurrentDirectory());
    Console.WriteLine(created ? "Created commit-linter.toml" : "commit-linter.toml already exists");
    return 0;
}

if (args is ["install", ..])
{
    var hookStatus = InstallHook(Directory.GetCurrentDirectory());
    if (hookStatus == "no-git")
    {
        Console.Error.WriteLine("error: could not find .git directory in the current working directory");
        return 1;
    }

    Console.WriteLine("Installed project-local Git commit-msg hook at .git/hooks/commit-msg");
    var created = InitProject(Directory.GetCurrentDirectory());
    Console.WriteLine(created ? "Created commit-linter.toml" : "commit-linter.toml already exists");
    return 0;
}

var binary = FindBinary();
if (binary is null)
{
    Console.Error.WriteLine("commit-linter-rs native binary was not found in this .NET tool package.");
    Console.Error.WriteLine("Reinstall the package or use a release that includes your platform binary.");
    return 1;
}

var startInfo = new ProcessStartInfo
{
    FileName = binary,
    UseShellExecute = false,
};

foreach (var argument in args)
{
    startInfo.ArgumentList.Add(argument);
}

try
{
    using var process = Process.Start(startInfo);
    if (process is null)
    {
        Console.Error.WriteLine("commit-linter-rs native binary was not found in this .NET tool package.");
        return 1;
    }

    process.WaitForExit();
    return process.ExitCode;
}
catch (Exception error) when (error is System.ComponentModel.Win32Exception or FileNotFoundException)
{
    Console.Error.WriteLine("commit-linter-rs native binary was not found in this .NET tool package.");
    return 1;
}

static bool InitProject(string projectRoot)
{
    var configPath = Path.Combine(projectRoot, ConfigFile);
    if (File.Exists(configPath))
    {
        return false;
    }

    File.WriteAllText(configPath, DefaultConfig);
    return true;
}

static string InstallHook(string projectRoot)
{
    var gitDir = Path.Combine(projectRoot, ".git");
    if (!Directory.Exists(gitDir))
    {
        return "no-git";
    }

    var hooksDir = Path.Combine(gitDir, "hooks");
    Directory.CreateDirectory(hooksDir);

    var hookPath = Path.Combine(hooksDir, "commit-msg");
    var hookBlock = $"""
{HookStart}
PROJECT_ROOT="$(dirname "$0")/../.."
cd "$PROJECT_ROOT"
dotnet tool run commit-linter-rs lint --edit "$1"
{HookEnd}
""";

    File.WriteAllText(hookPath, MergeHook(ReadFileIfExists(hookPath), hookBlock));

    if (!OperatingSystem.IsWindows())
    {
        File.SetUnixFileMode(
            hookPath,
            UnixFileMode.UserRead | UnixFileMode.UserWrite | UnixFileMode.UserExecute |
            UnixFileMode.GroupRead | UnixFileMode.GroupExecute |
            UnixFileMode.OtherRead | UnixFileMode.OtherExecute);
    }

    return "installed";
}

static string MergeHook(string? existingHook, string hookBlock)
{
    if (existingHook is null)
    {
        return $"#!/bin/sh\n{hookBlock}\n";
    }

    var cleaned = RemoveManagedBlock(existingHook).Replace(LegacyHookBlock, "").TrimEnd();
    return $"{cleaned}\n\n{hookBlock}\n";
}

static string RemoveManagedBlock(string hook)
{
    var start = hook.IndexOf(HookStart, StringComparison.Ordinal);
    var end = hook.IndexOf(HookEnd, StringComparison.Ordinal);
    if (start == -1 || end == -1 || end < start)
    {
        return hook;
    }

    return hook[..start] + hook[(end + HookEnd.Length)..];
}

static string? ReadFileIfExists(string path)
{
    return File.Exists(path) ? File.ReadAllText(path) : null;
}

static string? FindBinary()
{
    var configuredBinary = Environment.GetEnvironmentVariable("COMMIT_LINTER_RS_BIN");
    if (!string.IsNullOrWhiteSpace(configuredBinary) && File.Exists(configuredBinary))
    {
        return configuredBinary;
    }

    var bundledBinary = Path.Combine(AppContext.BaseDirectory, "vendor", BundledBinaryName());
    if (File.Exists(bundledBinary))
    {
        return bundledBinary;
    }

    return null;
}

static string BundledBinaryName()
{
    var arch = RuntimeInformation.ProcessArchitecture switch
    {
        Architecture.X64 => "x64",
        Architecture.Arm64 => "arm64",
        _ => RuntimeInformation.ProcessArchitecture.ToString().ToLowerInvariant(),
    };

    if (OperatingSystem.IsWindows())
    {
        return $"commit-linter-rs-windows-{arch}.exe";
    }

    if (OperatingSystem.IsMacOS())
    {
        return $"commit-linter-rs-macos-{arch}";
    }

    return $"commit-linter-rs-linux-{arch}";
}
