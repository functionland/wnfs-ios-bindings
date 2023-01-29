//
//  File.swift
//  
//
//  Created by Homayoun on 1/18/23.
//

import Foundation
import Wnfs
import CryptoKit
import CID
import Multihash
import Multicodec


private class WrapClosure<G, P> {
    fileprivate let get_closure: G
    fileprivate let put_closure: P
    init(get_closure: G, put_closure: P) {
        self.get_closure = get_closure
        self.put_closure = put_closure
    }
}

func cPutFn(bytes: UnsafePointer<UInt8>?, bytes_size: UnsafePointer<Int>?, result_size: UnsafeMutablePointer<Int>?, codec: Int64) -> UnsafePointer<UInt8>?{
    let bts = withUnsafePointer(to: &bytes[0]) {
        $0.withMemoryRebound(to: UInt8.self, capacity: bytes_size?.pointee!) { $0 }
    }
//            let hash = try Multihash(raw: bts, hashedWith: .sha2_256)
//            let cid = try CID(version: .v1, codec: .dag_cbor, multihash: hash)
    let cid = putFn(bts, try Codecs(codec))
    result_size?.pointee = cid.bytes.count
    let result: UnsafePointer<UInt8>? = UnsafePointer(cid)
    return nil
}

func cGetFn(cid: UnsafePointer<UInt8>?, cid_size: UnsafePointer<Int>?, result_size: UnsafeMutablePointer<Int>?) -> UnsafePointer<UInt8>? {
//    let bts = withUnsafePointer(to: &cid[0]) {
//        $0.withMemoryRebound(to: UInt8.self, capacity: cid_size?.pointee!) { $0 }
//    }
    let bts = Data(bytes: cid!, count: cid_size?.pointee!)
//            let hash = try Multihash(raw: bts, hashedWith: .sha2_256)
//            let cid = try CID(version: .v1, codec: .dag_cbor, multihash: hash)
    let data = getFn(bts)
    result_size?.pointee = data.bytes.count
    let result: UnsafePointer<UInt8>? = UnsafePointer(data)
    return nil
}

public class WnfsWrapper {
    var blockStoreInterface: UnsafeMutablePointer<BlockStoreInterface>?
    var x: Int
    init(putFn: @escaping ((_ data: Data, _ codec: Int64) -> Data), getFn: @escaping ((_ cid: Data) -> Data)) {
        self.x = 5
        self.blockStoreInterface = new_block_store_interface(cPutFn, cGetFn)
    }
    
    public func CreatePrivateForest() -> String? {
        let result = create_private_forest_native(self.blockStoreInterface)
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
        let ptr = Wnfs.create_root_dir_native(self.blockStoreInterface, wnfs_key_size!, wnfs_key_ptr!, makeCString(from: cid))
        
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
