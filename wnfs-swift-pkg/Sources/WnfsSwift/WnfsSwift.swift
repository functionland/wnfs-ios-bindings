//
//  File.swift
//  
//
//  Created by Homayoun on 1/18/23.
//

import Foundation
import Wnfs
import CryptoKit

public class WnfsWrapper {
    var dbPath: String
    init(dbPath: String){
        self.dbPath = dbPath
    }
    
    public func CreatePrivateForest() -> String? {
        let db_path = self.makeCString(from: self.dbPath)
        let result = create_private_forest_native(db_path)
        // FIXME: throw an error or something
        guard let ccid = result else {
            print("Empty cid response")
            return nil
        }
        let cid = String(cString: ccid)
        // Freeing the memory
        Wnfs.cstring_free(ccid)
        return cid
    }
    
    public func CreateRootDir(cid: String, wnfsKey: String) -> Wnfs.Config? {
        let msg = wnfsKey.data(using: .utf8)!
        let hashed = SHA256.hash(data: msg)
        var wnfs_key_ptr: UnsafePointer<UInt8>?
        var wnfs_key_size: Int?
        hashed.withUnsafeBytes { (unsafeBytes) in
            wnfs_key_ptr = unsafeBytes.bindMemory(to: UInt8.self).baseAddress!
            wnfs_key_size = unsafeBytes.count
        }
        let ptr = Wnfs.create_root_dir_native(self.dbPath, wnfs_key_size!, wnfs_key_ptr!, makeCString(from: cid))
        
        return self.unwrapConfigPtr(ptr: ptr)
    }
    
    private func unwrapConfigPtr(ptr: UnsafeMutablePointer<Config>?) -> Config? {
        var c = Config()
        c.cid = ptr?.pointee.cid
        c.private_ref = ptr?.pointee.private_ref
        config_free(ptr)
        return c
    }
    
    private func makeCString(from str: String) -> UnsafeMutablePointer<Int8> {
        let count = str.utf8.count + 1
        let result = UnsafeMutablePointer<Int8>.allocate(capacity: count)
        str.withCString { (baseAddress) in
            // func initialize(from: UnsafePointer<Pointee>, count: Int)
            result.initialize(from: baseAddress, count: count)
        }
        return result
    }
}
