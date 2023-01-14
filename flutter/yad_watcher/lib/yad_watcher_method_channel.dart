import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'yad_watcher_platform_interface.dart';

/// An implementation of [YadWatcherPlatform] that uses method channels.
class MethodChannelYadWatcher extends YadWatcherPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('yad/watcher');
  final eventsChannel = const EventChannel('yad/watcher/events');

  @override
  late Stream<dynamic> dataStream = eventsChannel.receiveBroadcastStream();

  @override
  Future<String?> getPlatformVersion() async {
    final version =
        await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }

  @override
  Future<String?> startWatch() async {
    final version = await methodChannel.invokeMethod<String>('startWatch');
    return version;
  }

  @override
  Future<String?> stopWatch() async {
    final version = await methodChannel.invokeMethod<String>('stopWatch');
    return version;
  }
}
