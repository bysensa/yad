//
//  NSRunningApplication.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation


extension NSRunningApplication {
    var appName: String {
        return localizedName ?? "Unknown"
    }
    
    var isBrowser: Bool {
        switch appName {
        case "Safari", "Google Chrome", "Chromium", "Opera", "Vivaldi", "Brave Browser", "Microsoft Edge", "Yandex": return true
        default: return false
        }
    }
}
