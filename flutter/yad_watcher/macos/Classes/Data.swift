//
//  Data.swift
//  yad_watcher
//
//  Created by Sergei Sen on 14.01.2023.
//

import Foundation

extension Data {
    static func from<T>(type: String, instance: T) -> YadResult<Data> where T : Encodable {
        let container = Container(type: type, value: instance);
        let res = Result { try JSONEncoder().encode(container) }
        return res.mapError { err in YadWatcherError.serialization(msg: err.localizedDescription)}
    }
    
    
    static func from<T>(type: String, instance: T?) -> YadResult<Data>  where T : Encodable {
        guard let instance = instance else {
            return .success("{\"type\": \"Unknown\"}".data(using: .utf8)!)
        }
        let container = Container(type: type, value: instance);
        let res = Result { try JSONEncoder().encode(container) }
        return res.mapError { err in YadWatcherError.serialization(msg: err.localizedDescription)}
    }
}

private class Container<T>: Encodable where T: Encodable {
    let type: String
    let value: T
    
    init(type: String, value: T) {
        self.type = type
        self.value = value
    }
}
