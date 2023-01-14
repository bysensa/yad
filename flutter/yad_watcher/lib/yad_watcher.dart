import 'dart:async';

import 'package:yad_watcher/model.dart';

import 'yad_watcher_platform_interface.dart';

class YadWatcher {
  Stream<Activity> get activities => YadWatcherPlatform.instance.dataStream
      .where((event) => event is String)
      .cast<String>()
      .map(Activity.from);

  Future<String?> getPlatformVersion() {
    return YadWatcherPlatform.instance.getPlatformVersion();
  }

  Future<void> startWatch() async {
    final res = await YadWatcherPlatform.instance.startWatch();
    print("start: ${Activity.from(res!)}");
  }

  Future<void> stopWatch() async {
    final res = await YadWatcherPlatform.instance.stopWatch();
    print("stop: ${Activity.from(res!)}");
  }
}
