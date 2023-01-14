import 'dart:async';
import 'dart:convert';

import 'package:deep_pick/deep_pick.dart';

abstract class Activity {
  factory Activity.from(String string) {
    try {
      final data = pick(jsonDecode(string));
      final type = data('type').asStringOrThrow();
      if (type == "Unknown") {
        return UnknownActivity();
      } else if (type == "ScreenLocked") {
        return ScreenLocked();
      } else if (type == "ScreenUnlocked") {
        return ScreenUnlocked();
      } else {
        final value = data('value');
        return ApplicationActivated.fromPick(value);
      }
    } catch (err, trace) {
      Zone.current.handleUncaughtError(err, trace);
      return UnknownActivity();
    }
  }
}

class ScreenLocked with Activity {}

class ScreenUnlocked with Activity {}

class UnknownActivity with Activity {}

class ApplicationActivated with Activity {
  final DateTime timestamp;
  final int pid;
  final String bundleId;
  final String name;
  final bool isBrowser;
  final Duration duration;

  const ApplicationActivated({
    required this.timestamp,
    required this.pid,
    required this.bundleId,
    required this.name,
    required this.isBrowser,
    required this.duration,
  });

  factory ApplicationActivated.fromPick(Pick pick) {
    final timestamp = DateTime.fromMillisecondsSinceEpoch(
      pick('timestamp').asIntOrThrow(),
    );
    final pid = pick('pid').asIntOrThrow();
    final bundleId = pick('bundleId').asStringOrThrow();
    final name = pick('name').asStringOrThrow();
    final isBrowser = pick('isBrowser').asBoolOrThrow();
    final duration = Duration(
      milliseconds: pick('durationInMs').asIntOrThrow(),
    );

    return ApplicationActivated(
      timestamp: timestamp,
      pid: pid,
      bundleId: bundleId,
      name: name,
      isBrowser: isBrowser,
      duration: duration,
    );
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApplicationActivated &&
          runtimeType == other.runtimeType &&
          timestamp == other.timestamp &&
          pid == other.pid &&
          bundleId == other.bundleId &&
          name == other.name &&
          duration == other.duration &&
          isBrowser == other.isBrowser;

  @override
  int get hashCode =>
      timestamp.hashCode ^
      pid.hashCode ^
      bundleId.hashCode ^
      name.hashCode ^
      duration.hashCode ^
      isBrowser.hashCode;

  @override
  String toString() {
    return 'ApplicationActivated {'
        'timestamp: $timestamp, '
        'pid: $pid, '
        'bundleId: $bundleId, '
        'name: $name, '
        'isBrowser: $isBrowser,'
        'duration: $duration'
        '}';
  }
}
