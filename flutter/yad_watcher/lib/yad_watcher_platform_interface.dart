import 'dart:async';

import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'yad_watcher_method_channel.dart';

abstract class YadWatcherPlatform extends PlatformInterface {
  /// Constructs a YadWatcherPlatform.
  YadWatcherPlatform() : super(token: _token);

  static final Object _token = Object();

  static YadWatcherPlatform _instance = MethodChannelYadWatcher();

  /// The default instance of [YadWatcherPlatform] to use.
  ///
  /// Defaults to [MethodChannelYadWatcher].
  static YadWatcherPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [YadWatcherPlatform] when
  /// they register themselves.
  static set instance(YadWatcherPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Stream<dynamic> get dataStream;

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }

  Future<String?> startWatch() {
    throw UnimplementedError('startWatch() has not been implemented.');
  }

  Future<String?> stopWatch() {
    throw UnimplementedError('stopWatch() has not been implemented.');
  }
}
