//
//  YadWatcherError.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation

enum YadWatcherError: Error {
    case unimplemented
    case serialization(msg: String)
}

extension YadWatcherError {
    var isFatal: Bool {
        if case YadWatcherError.serialization = self { return true }
        if case YadWatcherError.unimplemented = self { return true }
        else { return false }
    }
}


// For each error type return the appropriate description
extension YadWatcherError: CustomStringConvertible {
    public var description: String {
        switch self {
        case .unimplemented:
            return "Unimplemented"
        case .serialization:
            return "Serialization"
        }
    }
}

// For each error type return the appropriate localized description
extension YadWatcherError: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .unimplemented:
            return NSLocalizedString(
                "The code block you try to cal is unimplemented",
                comment: "Unimplemented"
            )
            
        case .serialization(let msg):
            return NSLocalizedString(
                "Receive error during serialization: \(msg)",
                comment: "Serialization"
            )
        }
    }
}
