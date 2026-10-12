import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

const _configFile = 'commit-linter.toml';
const _hookStart = '# commit-linter-rs start';
const _hookEnd = '# commit-linter-rs end';
const _legacyHookBlock = 'commit-linter-rs lint --edit "\$1"';

const _defaultConfig = '''max_header_length = 100
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
''';

Future<int> run(List<String> arguments) async {
  final projectRoot = Directory.current;

  if (arguments.firstOrNull == 'init') {
    final created = _initProject(projectRoot);
    stdout.writeln(
      created ? 'Created $_configFile' : '$_configFile already exists',
    );
    return 0;
  }

  if (arguments.firstOrNull == 'install') {
    if (!_installHook(projectRoot)) {
      stderr.writeln(
        'error: could not find .git directory in the current working directory',
      );
      return 1;
    }

    stdout.writeln(
      'Installed project-local Git commit-msg hook at .git/hooks/commit-msg',
    );
    final created = _initProject(projectRoot);
    stdout.writeln(
      created ? 'Created $_configFile' : '$_configFile already exists',
    );
    return 0;
  }

  final binary = await _findBinary();
  if (binary == null) {
    stderr.writeln(
      'commit-linter-rs native binary was not found in this Dart package.',
    );
    stderr.writeln(
      'Reinstall the package or use a release that includes your platform binary.',
    );
    return 1;
  }

  if (!Platform.isWindows) {
    await Process.run('chmod', ['+x', binary]);
  }

  try {
    final process = await Process.start(
      binary,
      arguments,
      mode: ProcessStartMode.inheritStdio,
    );
    return await process.exitCode;
  } on ProcessException catch (error) {
    stderr.writeln(error.message);
    return 1;
  }
}

bool _initProject(Directory projectRoot) {
  final config = File('${projectRoot.path}${Platform.pathSeparator}$_configFile');
  if (config.existsSync()) return false;

  config.writeAsStringSync(_defaultConfig);
  return true;
}

bool _installHook(Directory projectRoot) {
  final separator = Platform.pathSeparator;
  final gitDirectory = Directory('${projectRoot.path}${separator}.git');
  if (!gitDirectory.existsSync()) return false;

  final hooksDirectory = Directory('${gitDirectory.path}${separator}hooks')
    ..createSync(recursive: true);
  final hook = File('${hooksDirectory.path}${separator}commit-msg');
  final block = '''$_hookStart
PROJECT_ROOT="\$(dirname "\$0")/../.."
cd "\$PROJECT_ROOT"
dart run commit_linter_rs lint --edit "\$1"
$_hookEnd
''';

  final existing = hook.existsSync() ? hook.readAsStringSync() : null;
  hook.writeAsStringSync(_mergeHook(existing, block));
  if (!Platform.isWindows) {
    Process.runSync('chmod', ['+x', hook.path]);
  }
  return true;
}

String _mergeHook(String? existing, String block) {
  if (existing == null) return '#!/bin/sh\n$block';

  final cleaned = _removeManagedBlock(existing)
      .replaceAll(_legacyHookBlock, '')
      .trimRight();
  return '$cleaned\n\n$block';
}

String _removeManagedBlock(String hook) {
  final start = hook.indexOf(_hookStart);
  final end = hook.indexOf(_hookEnd);
  if (start == -1 || end == -1 || end < start) return hook;

  return hook.substring(0, start) + hook.substring(end + _hookEnd.length);
}

Future<String?> _findBinary() async {
  final configured = Platform.environment['COMMIT_LINTER_RS_BIN'];
  if (configured != null && File(configured).existsSync()) return configured;

  final uri = await Isolate.resolvePackageUri(
    Uri.parse('package:commit_linter_rs/vendor/${_bundledBinaryName()}'),
  );
  if (uri == null || uri.scheme != 'file') return null;

  final path = uri.toFilePath();
  return File(path).existsSync() ? path : null;
}

String _bundledBinaryName() {
  final architecture = _architecture();
  if (Platform.isWindows) return 'commit-linter-rs-windows-$architecture.exe';
  if (Platform.isMacOS) return 'commit-linter-rs-macos-$architecture';
  if (Platform.isLinux) return 'commit-linter-rs-linux-$architecture';
  return 'commit-linter-rs-${Platform.operatingSystem}-$architecture';
}

String _architecture() {
  final abi = Abi.current().toString().toLowerCase();
  if (abi.contains('arm64')) return 'arm64';
  if (abi.contains('x64')) return 'x64';

  return abi.split('_').last;
}

extension on List<String> {
  String? get firstOrNull => isEmpty ? null : first;
}
