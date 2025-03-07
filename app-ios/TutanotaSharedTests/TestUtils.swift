import Mockingbird
import Testing
import XCTest
import Combine

func initMockingbird() {
	struct SwiftTestFailer: TestFailer {
		func fail(message: String, isFatal: Bool, file: StaticString, line: UInt) {
			// swiftlint:disable optional_data_string_conversion
			let filename = file.withUTF8Buffer { String(decoding: $0, as: UTF8.self) }
			// swiftlint:enable optional_data_string_conversion
			Issue.record(
				Comment(stringLiteral: message),
				sourceLocation: Testing.SourceLocation(fileID: filename, filePath: filename, line: Int(line), column: 0)
			)
		}
	}

	// FIXME: we need to fix up all the other tests
	swizzleTestFailer(SwiftTestFailer())
}

extension XCTest {
	func initMockingbird() {
		class StandardTestFailer: TestFailer {
			func fail(message: String, isFatal: Bool, file: StaticString, line: UInt) {
				guard isFatal else {
					return XCTFail(message, file: file, line: line)
				}

				// we don't do this
				// Raise an Objective-C exception to stop the test runner.
				//			MKBStopTest(message)

				// Test execution should usually be stopped by this point.
				fatalError(message)
			}
		}
	}
}

struct ResolvableFuture {
	private let future: Future<Void, Never>
	private let promise: Future<Void, Never>.Promise

	init() {
		var promise: Future<Void, Never>.Promise!
		let future = Future { p in promise = p }
		self.future = future
		self.promise = promise
	}

	var value: Void { get async { await self.future.value } }

	func resolve() { self.promise(.success(())) }
}
