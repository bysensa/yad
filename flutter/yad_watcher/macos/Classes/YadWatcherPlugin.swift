import Combine
import Cocoa
import FlutterMacOS

public class YadWatcherPlugin: NSObject, FlutterPlugin {
    public static func register(with registrar: FlutterPluginRegistrar) {
        let channel = FlutterMethodChannel(name: "yad/watcher", binaryMessenger: registrar.messenger)
        let eventHandler = EventChannelHandler(name: "yad/watcher/events", messenger: registrar.messenger)
        let watcher = YadWatcher(eventHandler: eventHandler)
        let instance = YadWatcherPlugin(watcher: watcher)
        
        registrar.addMethodCallDelegate(instance, channel: channel)
    }
    
    private let watcher: YadWatcher
    
    init(watcher: YadWatcher) {
        self.watcher = watcher
    }
    
    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        switch call.method {
        case "getPlatformVersion":
            result("macOS " + ProcessInfo.processInfo.operatingSystemVersionString)
        case "startWatch":
            watcher.startWatch().flush(into: result)
        case "stopWatch":
            watcher.startWatch().flush(into: result)
        default:
            result(FlutterMethodNotImplemented)
        }
    }
}

