//
//  ApplicationActivation.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation


struct ApplicationActivated: Codable {
    var timestamp: Int64
    var pid: Int32
    var bundleId: String
    var name: String
    var isBrowser: Bool
    var durationInMs: Int64
    
    public static func from(_ runningApplication: NSRunningApplication) -> ApplicationActivated {
        ApplicationActivated(
            timestamp: Date().millisecondsSince1970,
            pid: runningApplication.processIdentifier,
            bundleId: runningApplication.bundleIdentifier ?? "Unknown",
            name: runningApplication.localizedName ?? "Unknown",
            isBrowser: runningApplication.isBrowser,
            durationInMs: 0
        )
    }
}
