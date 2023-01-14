//
//  YadWatcher.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation
import Cocoa


class YadWatcher {
    private let eventHandler: EventChannelHandler
    private let workspace: NSWorkspace
    private let workspaceNotificationCenter : NotificationCenter
    private let distributedNotificationCenter : DistributedNotificationCenter
    
    private var applicationActivationObserver: NSObjectProtocol?
    private var screenLockObserver: NSObjectProtocol?
    private var screenUnlockObserver: NSObjectProtocol?
    
    private var lastRunningApplication: ApplicationActivated?
    
    init(eventHandler: EventChannelHandler) {
        self.eventHandler = eventHandler
        self.workspace = NSWorkspace.shared
        self.workspaceNotificationCenter = workspace.notificationCenter
        self.distributedNotificationCenter = DistributedNotificationCenter.default()
    }
    
    // MARK: Public Methods
    
    public func startWatch() -> YadResult<Data> {
        setupApplicationActivationObserver()
        setupScreenLockObserver()
        setupScreenUnlockObserver()
        let activatedNow = currentApplicationActivated()
        lastRunningApplication = activatedNow
        return Data.from(type: "ApplicationActivated", instance: activatedNow)
    }
    
    public func stopWatch() -> YadResult<Data> {
        removeObserverss()
        let activatedNow = currentApplicationActivated()
        var lastActivated = lastRunningApplication
        lastRunningApplication = activatedNow
        
        if activatedNow != nil && lastActivated != nil {
            let elapsed = activatedNow!.timestamp - lastActivated!.timestamp
            lastActivated?.durationInMs = elapsed
        }
        
        return Data.from(type: "ApplicationActivated", instance: lastActivated)
    }
    
    private func currentApplicationActivated() -> ApplicationActivated? {
        let maybeRunningApplication = workspace.frontmostApplication
        if let runningApplication = maybeRunningApplication {
            return ApplicationActivated.from(runningApplication)
        }
        return nil
    }
    
    // MARK: Observers setters
    
    private func setupApplicationActivationObserver() {
        applicationActivationObserver = workspaceNotificationCenter.addObserver(
            forName: NSWorkspace.didActivateApplicationNotification,
            object: nil,
            queue: nil,
            using: applicationActivated
        )
    }
    
    private func setupScreenLockObserver() {
        screenLockObserver = distributedNotificationCenter.addObserver(
            forName: Notification.Name("com.apple.screenIsLocked"),
            object: nil,
            queue: nil,
            using: screenLocked
        )
    }
    
    private func setupScreenUnlockObserver() {
        screenUnlockObserver = distributedNotificationCenter.addObserver(
            forName: Notification.Name("com.apple.screenIsUnlocked"),
            object: nil,
            queue: nil,
            using: screenUnlocked
        )
    }
    
    // MARK: Observers remover
    private func removeObserverss() {
        workspaceNotificationCenter.removeObserver(applicationActivationObserver as Any)
        distributedNotificationCenter.removeObserver(screenLockObserver as Any)
        distributedNotificationCenter.removeObserver(screenUnlockObserver as Any)
    }
    
    // MARK: Notifications Handlers
    
    private func screenLocked(notification: Notification) {
        let dataString = "{\"type\": \"ScreenLocked\"}"
        try? eventHandler.success(event: dataString)
    }
    
    private func screenUnlocked(notification: Notification) {
        let dataString = "{\"type\": \"ScreenUnlocked\"}"
        try? eventHandler.success(event: dataString)
    }
    
    private func applicationActivated(notification: Notification) {
        if notification.userInfo != nil {
            guard let userInfo = notification.userInfo else {
                return
            }
            userInfo.values
                .filter { $0 is NSRunningApplication }
                .map { $0 as? NSRunningApplication }
                .forEach { info in
                    guard let info = info else {
                        return
                    }
                    let activatedNow = ApplicationActivated.from(info)
                    guard var lastActivated = lastRunningApplication else {
                        lastRunningApplication = activatedNow
                        return
                    }
                    lastRunningApplication = activatedNow
                    let elapsedOfLastActivated = activatedNow.timestamp - lastActivated.timestamp
                    lastActivated.durationInMs = elapsedOfLastActivated
                    let res = Data.from(type: "ApplicationActivated", instance: lastActivated)
                    switch res {
                    case .success(let value):
                        try? eventHandler.success(event: String(data: value, encoding: .utf8))
                    case .failure(let err):
                        try? eventHandler.error(code: err.description, message: err.localizedDescription, details: nil)
                    }
                    
                }
        }
    }
}
