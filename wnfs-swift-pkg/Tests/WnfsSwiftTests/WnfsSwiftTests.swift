//
//  Test.swift
//
//  Created by Homayoun on 1/16/23.
//
import func XCTest.XCTAssertEqual
import Foundation
import XCTest
import CryptoKit
@testable import WnfsSwift

class MockSession {

    static let sharedInstance = MockSession()

    var myData = Dictionary<String,AnyObject>()
}

public func mockFulaGet(_cid: UnsafePointer<UInt8>?, cid_size: UnsafePointer<Int>?, result_size: UnsafeMutablePointer<Int>?) -> UnsafePointer<UInt8>? {
    let myData = MockSession.sharedInstance.myData as Dictionary
    let hospitalDict = myData["hospital"]


    if let name = hospitalDict?["name"] as? String {
        print(name)
    }
}

public func mockFulaPut(_bytes: UnsafePointer<UInt8>?, bytes_size: UnsafePointer<Int>?, result_size: UnsafeMutablePointer<Int>?, codec: Int64) -> UnsafePointer<UInt8>? {
    var dict = Dictionary<String,AnyObject>()
    dict["pid"] = "123"
    dict["hospital"] = ["id":"234", "name":"newHorizon", "type":"nursing home"]

    MockSession.sharedInstance.myData = dict
}

final class WnfsSwiftTest: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testOverall() throws {
        let wnfsWrapper = WnfsWrapper(dbPath: NSTemporaryDirectory())
        let cid = wnfsWrapper.CreatePrivateForest()
        assert(cid != nil)
        
        let config =  wnfsWrapper.CreateRootDir(cid: cid!, wnfsKey: "test")
        assert(config != nil)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }
}
