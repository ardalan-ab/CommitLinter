import 'dart:io';

import 'package:commit_linter_rs/commit_linter_rs.dart';

Future<void> main(List<String> arguments) async {
  exitCode = await run(arguments);
}

