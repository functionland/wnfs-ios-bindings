//
//  Test.swift
//
//  Created by Homayoun on 1/16/23.
//
import func XCTest.XCTAssertEqual
import Foundation
import XCTest
import CryptoKit
import CID
import Multihash
import Multicodec
@testable import WnfsSwift

class MockSession {

    static let sharedInstance = MockSession()

    var myData = Dictionary<Data,Data>()
}

public func mockFulaGet(_ cid: Data?) -> Data? {

    if let data = MockSession.sharedInstance.myData[cid!] {
        return data
    }
    return nil
}

public func mockFulaPut(_ data: Data?, _ _codec: Int64) -> Data? {
    let codec: Codecs
    do{
        codec = try Codecs(_codec)
    } catch let error{
        print(error.localizedDescription)
        return nil
    }
    let hash: Multihash
    let cid: CID
    do{
        hash = try Multihash(raw: data!, hashedWith: .sha2_256)
    } catch let error{
        print(error.localizedDescription)
        return nil
    }
    do{
        cid = try CID(version: .v1, codec: codec, multihash: hash)
    } catch let error{
        print(error.localizedDescription)
        return nil
    }

    MockSession.sharedInstance.myData[cid.rawData] = data!
    print(cid.toBaseEncodedString)
    print(cid.rawData.toHexString())
    return Data(cid.rawData)
}

final class WnfsSwiftTest: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testCID() throws {
        let mh = try Multihash(raw: "abc", hashedWith: .sha2_256)
        let cid = try CID(version: .v1, codec: .dag_cbor, multihash: mh)

        assert(cid.toBaseEncodedString == "bafyreif2pall7dybz7vecqka3zo24irdwabwdi4wc55jznaq75q7eaavvu")
        let cid_recreated = try CID(cid.rawData.bytes)
        assert(cid_recreated.toBaseEncodedString == cid.toBaseEncodedString)
    }
    
    func testOverall() throws {
        let wnfsWrapper = WnfsWrapper(putFn: mockFulaPut, getFn: mockFulaGet)
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
