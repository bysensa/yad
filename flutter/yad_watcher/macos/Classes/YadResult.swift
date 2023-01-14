//
//  YadResult.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation
import FlutterMacOS

typealias YadResult<T> = Result<T, YadWatcherError>

extension YadResult where Success == Data, Failure == YadWatcherError {
    internal func flush(into result: FlutterResult) {
        switch self {
        case .success(let data):
            result(String(data: data, encoding: .utf8))
        case .failure(let error):
            result(FlutterError(code: error.description, message: error.localizedDescription, details: nil))
        }
    }
}

extension YadResult where Success == Bool, Failure == YadWatcherError {
    internal func flush(into result: FlutterResult) {
        switch self {
        case .success(let data):
            result(data)
        case .failure(let error):
            result(FlutterError(code: error.description, message: error.localizedDescription, details: nil))
        }
    }
}
