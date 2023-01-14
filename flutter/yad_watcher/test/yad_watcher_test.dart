import 'package:flutter_test/flutter_test.dart';
import 'package:yad_watcher/yad_watcher.dart';
import 'package:yad_watcher/yad_watcher_platform_interface.dart';
import 'package:yad_watcher/yad_watcher_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockYadWatcherPlatform
    with MockPlatformInterfaceMixin
    implements YadWatcherPlatform {
  @override
  Future<String?> getPlatformVersion() => Future.value('42');

  @override
  Future<String?> startWatch() {
    throw UnimplementedError();
  }

  @override
  Future<String?> stopWatch() {
    throw UnimplementedError();
  }

  @override
  Stream get dataStream => throw UnimplementedError();
}

void main() {
  final YadWatcherPlatform initialPlatform = YadWatcherPlatform.instance;

  test('$MethodChannelYadWatcher is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelYadWatcher>());
  });

  test('getPlatformVersion', () async {
    YadWatcher yadWatcherPlugin = YadWatcher();
    MockYadWatcherPlatform fakePlatform = MockYadWatcherPlatform();
    YadWatcherPlatform.instance = fakePlatform;

    expect(await yadWatcherPlugin.getPlatformVersion(), '42');
  });
}
