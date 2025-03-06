import Mockingbird
import TutanotaSharedFramework
import XCTest

class NotificationsHandlerTest : XCTestCase {
	private var alarmManager: AlarmManagerMock!
	private var notificationStorage: NotificationStorage!
	private var notificationsHandler: NotificationsHandler!
	private var httpClient: HttpClientMock!

	override func setUp() async throws {
		httpClient = mock(HttpClient.self)
		alarmManager = mock(AlarmManager.self)
			.initialize(
				alarmPersistor: mock(AlarmPersistor.self),
				alarmCryptor: mock(AlarmCryptor.self),
				alarmScheduler: mock(AlarmScheduler.self),
				alarmCalculator: mock(AlarmCalculator.self))
		notificationStorage = mock(NotificationStorage.self)
			.initialize(userPreferencesProvider: mock(UserPreferencesProvider.self))
		notificationsHandler = NotificationsHandler(alarmManager: alarmManager, notificationStorage: notificationStorage, httpClient: httpClient)
	}

	func test_downloadsAndProcessesAlarms_noAlarms() async throws {
		let sseInfo = SSEInfo(pushIdentifier: "pushIdentifier", sseOrigin: "sseorigin.com", userIds: ["userId"])
		given(notificationStorage.sseInfo).willReturn(sseInfo)
		let notification = #"{"lastProcessedNotificationId": "newLastId", "alarmNotifications": []}"#
		let data = notification.data(using: .utf8)!

		let response = HTTPURLResponse()
		given(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil))
			.willReturn((data, response))

		await withCheckedContinuation { (cont: CheckedContinuation<Void, Never>) in
			notificationsHandler.fetchMissedNotifications { _ in cont.resume(returning: ()) }
		}

		verify(notificationStorage.lastMissedNotificationCheckTime).wasCalled()

		verify(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil)).wasCalled()
		verify(alarmManager.processNewAlarms([])).wasCalled()
	}
}
