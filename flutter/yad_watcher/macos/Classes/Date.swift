//
//  Date.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation



extension Date {
    var millisecondsSince1970: Int64 {
        Int64((self.timeIntervalSince1970 * 1000.0).rounded())
    }
}

