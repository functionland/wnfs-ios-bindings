//
//  File.swift
//  
//
//  Created by Homayoun on 1/18/23.
//
import Foundation
import Wnfs
import CryptoKit

private class WrapClosure<G, P> {
    fileprivate let get_closure: G
    fileprivate let put_closure: P
    init(get_closure: G, put_closure: P) {
        self.get_closure = get_closure
        self.put_closure = put_closure
    }
}

func toData(ptr: UnsafePointer<UInt8>?, size: UnsafePointer<Int>?) -> Data? {
    guard let count = size else {
        return nil
    }
    let buffer = UnsafeBufferPointer(start: ptr, count: count.pointee)
    return Data(buffer: buffer)
}

public class WnfsWrapper {
    var blockStoreInterface: BlockStoreInterface
    init(putFn: @escaping ((_ data: Data?, _ codec: Int64) -> Data?), getFn: @escaping ((_ cid: Data?) -> Data?)) {
        // step 1
        let wrappedClosure = WrapClosure(get_closure: getFn, put_closure: putFn)
        let userdata = Unmanaged.passRetained(wrappedClosure).toOpaque()
        
        // step 2
        let cPutFn: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<UInt8>?, UnsafePointer<Int>?, Int64) -> UnsafePointer<SwiftData>? = { (_ userdata: UnsafeMutableRawPointer?, _ bytes: UnsafePointer<UInt8>?, _ bytes_size: UnsafePointer<Int>?, _ codec: Int64) -> UnsafePointer<SwiftData>? in
            let wrappedClosure: WrapClosure< (_ cid: Data?) -> Data? , (_ data: Data?, _ codec: Int64) -> Data?> = Unmanaged.fromOpaque(userdata!).takeUnretainedValue()
            let bts = toData(ptr: bytes, size: bytes_size)
            guard let cid = wrappedClosure.put_closure(bts, codec) else{
                return nil
            }
            print(cid.map { String(format: "%02hhx", $0) }.joined())
            var ptr = UnsafeMutablePointer<UInt8>.allocate(capacity: cid.count)
            cid.copyBytes(to: ptr, count: cid.count)
            let swiftData = SwiftData(ptr: UnsafePointer<UInt8>(ptr), count: cid.count)
            let _result = UnsafeMutablePointer<SwiftData>.allocate(capacity: 1)
            _result.initialize(to: swiftData)
            let result: UnsafePointer<SwiftData>? = UnsafePointer(_result)
            return result
        }
        
        // step 3
        let cGetFn: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<UInt8>?, UnsafePointer<Int>?) -> UnsafePointer<SwiftData>? = { (_ userdata: UnsafeMutableRawPointer?, _ cid: UnsafePointer<UInt8>?, _ cid_size: UnsafePointer<Int>?) -> UnsafePointer<SwiftData>? in
            let wrappedClosure: WrapClosure< (_ cid: Data?) -> Data? , (_ data: Data?, _ codec: Int64) -> Data?> = Unmanaged.fromOpaque(userdata!).takeUnretainedValue()
            let bts = toData(ptr: cid, size: cid_size)
            guard let data = wrappedClosure.get_closure(bts) else{
                return nil
            }

            var ptr = UnsafeMutablePointer<UInt8>.allocate(capacity: data.count)
            data.copyBytes(to: ptr, count: data.count)
            let swiftData = SwiftData(ptr: UnsafePointer<UInt8>(ptr), count: data.count)
            let _result = UnsafeMutablePointer<SwiftData>.allocate(capacity: 1)
            _result.initialize(to: swiftData)
            let result: UnsafePointer<SwiftData>? = UnsafePointer(_result)
            return result
        }
        
        let cDeallocFn: @convention(c) (UnsafePointer<SwiftData>?) -> Void = { (_ data: UnsafePointer<SwiftData>?) in
            data?.deallocate()
        }

        self.blockStoreInterface = BlockStoreInterface(userdata: userdata, put_fn: cPutFn, get_fn: cGetFn, dealloc: cDeallocFn)
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
        let ptr = create_root_dir_native(self.blockStoreInterface, wnfs_key_size!, wnfs_key_ptr!, makeCString(from: cid))
        
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
