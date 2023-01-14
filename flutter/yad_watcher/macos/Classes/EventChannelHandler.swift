//
//  EventChannelHandler.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation
import FlutterMacOS

public class EventChannelHandler: NSObject, FlutterStreamHandler {
    private var eventSink: FlutterEventSink?
    
    public var hasListeners: Bool {
        return eventSink != nil
    }

    public func onListen(withArguments arguments: Any?, eventSink events: @escaping FlutterEventSink) -> FlutterError? {
        self.eventSink = events
        return nil
    }

    public func onCancel(withArguments arguments: Any?) -> FlutterError? {
        eventSink = nil
        return nil
    }

    public init(name: String, messenger: FlutterBinaryMessenger) {
        super.init()
        let eventChannel = FlutterEventChannel(name: name, binaryMessenger: messenger)
        eventChannel.setStreamHandler(self)
    }


    public func success(event: Any?) throws {
        if eventSink != nil {
            eventSink?(event)
        }
    }

    public func error(code: String, message: String?, details: Any? = nil) {
        if eventSink != nil {
            eventSink?(FlutterError(code: code, message: message, details: details))
        }
    }

}
