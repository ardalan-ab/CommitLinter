using System.Diagnostics;

var binary = FindBinary();
if (binary is null)
{
    Console.Error.WriteLine("commit-linter-rs binary was not found on PATH.");
    Console.Error.WriteLine("Install the Rust CLI binary first or download it from GitHub Releases.");
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
        Console.Error.WriteLine("commit-linter-rs binary was not found on PATH.");
        Console.Error.WriteLine("Install the Rust CLI binary first or download it from GitHub Releases.");
        return 1;
    }

    process.WaitForExit();
    return process.ExitCode;
}
catch (Exception error) when (error is System.ComponentModel.Win32Exception or FileNotFoundException)
{
    Console.Error.WriteLine("commit-linter-rs binary was not found on PATH.");
    Console.Error.WriteLine("Install the Rust CLI binary first or download it from GitHub Releases.");
    return 1;
}

static string? FindBinary()
{
    var ownPath = Environment.ProcessPath;
    var ownFullPath = ownPath is null ? null : Path.GetFullPath(ownPath);
    var names = OperatingSystem.IsWindows()
        ? new[] { "commit-linter-rs.exe", "commit-linter-rs.cmd", "commit-linter-rs.bat" }
        : new[] { "commit-linter-rs" };

    foreach (var directory in (Environment.GetEnvironmentVariable("PATH") ?? "").Split(Path.PathSeparator))
    {
        if (string.IsNullOrWhiteSpace(directory))
        {
            continue;
        }

        foreach (var name in names)
        {
            var candidate = Path.Combine(directory, name);
            if (!File.Exists(candidate))
            {
                continue;
            }

            var candidateFullPath = Path.GetFullPath(candidate);
            if (!string.Equals(candidateFullPath, ownFullPath, StringComparison.OrdinalIgnoreCase))
            {
                return candidate;
            }
        }
    }

    return null;
}
