//
//  WNFSDemoTests.swift
//  WNFSDemoTests
//
//  Created by Homayoun on 1/16/23.
//

import func XCTest.XCTAssertEqual
import Foundation
import XCTest
import CryptoKit
@testable import WNFSDemo
import Wnfs

final class WNFSDemoTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testExample() throws {
        let tmpdir = NSTemporaryDirectory()
        let db_path = makeCString(from: tmpdir)
        let result = Wnfs.create_private_forest_native(db_path)
        // FIXME: throw an error
        guard let ccid = result else {
            print("Did not find a cat")
            return
        }
        let cid = String(cString: ccid)
        
        // Defining wnfs key
        let msg = "test".data(using: .utf8)!
        let hashed = SHA256.hash(data: msg)
        var wnfs_key_ptr: UnsafePointer<UInt8>?
        var wnfs_key_size: Int?
        hashed.withUnsafeBytes { (unsafeBytes) in
            wnfs_key_ptr = unsafeBytes.bindMemory(to: UInt8.self).baseAddress!
            wnfs_key_size = unsafeBytes.count
        }
        let config = Wnfs.create_root_dir_native(db_path, wnfs_key_size!, wnfs_key_ptr!, ccid)
        assert(config != nil)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }
    
    func makeCString(from str: String) -> UnsafeMutablePointer<Int8> {
        let count = str.utf8.count + 1
        let result = UnsafeMutablePointer<Int8>.allocate(capacity: count)
        str.withCString { (baseAddress) in
            // func initialize(from: UnsafePointer<Pointee>, count: Int)
            result.initialize(from: baseAddress, count: count)
        }
        return result
    }

}
