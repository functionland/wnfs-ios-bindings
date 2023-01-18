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
