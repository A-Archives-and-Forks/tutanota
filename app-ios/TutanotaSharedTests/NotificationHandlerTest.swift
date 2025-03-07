import Combine
import Mockingbird
import Testing
import TutanotaSharedFramework

struct NotificationsHandlerTest {
	private var alarmManager: AlarmManagerMock
	private var notificationStorage: NotificationStorageMock
	private var notificationsHandler: NotificationsHandler
	private let dateProvider = mock(DateProvider.self)
	private var httpClient: HttpClientMock! = mock(HttpClient.self)

	let sseInfo = SSEInfo(pushIdentifier: "pushIdentifier", sseOrigin: "sseorigin.com", userIds: ["userId"])

	init() {
		initMockingbird()
		alarmManager = mock(AlarmManager.self)
			.initialize(
				alarmPersistor: mock(AlarmPersistor.self),
				alarmCryptor: mock(AlarmCryptor.self),
				alarmScheduler: mock(AlarmScheduler.self),
				alarmCalculator: mock(AlarmCalculator.self)
			)
		notificationStorage = mock(NotificationStorage.self).initialize(userPreferencesProvider: mock(UserPreferencesProvider.self))
		notificationsHandler = NotificationsHandler(
			alarmManager: alarmManager,
			notificationStorage: notificationStorage,
			httpClient: httpClient,
			dateProvider: dateProvider
		)

		given(dateProvider.now).willReturn(Date.init(timeIntervalSince1970: 1))
	}

	@Test func downloadsAndProcessesAlarms_noAlarms() async throws {
		given(notificationStorage.sseInfo).willReturn(sseInfo)
		let newLastProcessedNotificationId = "newLastProcessedNotificationId"
		let notification = MissedNotification(alarmNotifications: [], lastProcessedNotificationId: newLastProcessedNotificationId)
		let data = try! JSONEncoder().encode(notification)

		let response = HTTPURLResponse()
		given(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil)).willReturn((data, response))

		await notificationsHandler.fetchMissedNotifications()

		verify(notificationStorage.lastMissedNotificationCheckTime).wasCalled()

		verify(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil)).wasCalled()
		verify(alarmManager.processNewAlarms([])).wasCalled()
		verify(notificationStorage.lastProcessedNotificationId = newLastProcessedNotificationId).wasCalled()
	}

	@Test func downloadsAndProcessesAlarms_WithAlarms() async throws {
		given(notificationStorage.sseInfo).willReturn(sseInfo)
		let alarm = EncryptedAlarmNotification(
			operation: .Create,
			summary: "",
			eventStart: "",
			eventEnd: "",
			alarmInfo: EncryptedAlarmInfo(alarmIdentifier: "", trigger: ""),
			repeatRule: nil,
			notificationSessionKeys: [],
			user: ""
		)
		let notification = MissedNotification(alarmNotifications: [alarm], lastProcessedNotificationId: "newLastProcessedNotificationId")
		let data = try! JSONEncoder().encode(notification)

		let response = HTTPURLResponse()
		given(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil)).willReturn((data, response))

		await notificationsHandler.fetchMissedNotifications()

		verify(alarmManager.processNewAlarms([alarm])).wasCalled()
	}

	@Test func downloadsAndProcessesAlarms_skipsAlreadyFetched() async throws {
		given(notificationStorage.sseInfo).willReturn(sseInfo)
		let oldLastId = "oldLastId"
		given(notificationStorage.lastProcessedNotificationId).willReturn(oldLastId)
		let notification = MissedNotification(alarmNotifications: [], lastProcessedNotificationId: "newLastProcessedNotificationId")
		let data = try! JSONEncoder().encode(notification)
		let response = HTTPURLResponse()
		// It's hard to control the order of invocations so we don't do the parallel invocations and instead
		// just check that the time of the request is before the last response and then skip the request.
		given(notificationStorage.lastMissedNotificationCheckTime).willReturn(Date(timeIntervalSince1970: 20))
		given(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil)).willReturn((data, response))
		given(dateProvider.now).willReturn(Date(timeIntervalSince1970: 10))

		// do two calls in parallel
		await notificationsHandler.fetchMissedNotifications()

		verify(await httpClient.fetch(url: any(), method: any(), headers: any(), body: any())).wasNeverCalled()
	}

	@Test func downloadsAndProcessesAlarms_callIfReceivedWhenInflight() async throws {
		given(notificationStorage.sseInfo).willReturn(sseInfo)
		let oldLastId = "oldLastId"
		given(notificationStorage.lastProcessedNotificationId).willReturn(oldLastId)
		let notification = MissedNotification(alarmNotifications: [], lastProcessedNotificationId: "newLastProcessedNotificationId")
		let data = try! JSONEncoder().encode(notification)
		let response = HTTPURLResponse()

		given(notificationStorage.lastMissedNotificationCheckTime).willReturn(Date(timeIntervalSince1970: 10))
		given(await httpClient.fetch(url: any(), method: .get, headers: any(), body: nil)).willReturn((data, response))
		// call was scheduled after the last response, do make the call
		given(dateProvider.now).willReturn(Date(timeIntervalSince1970: 20))

		// do two calls in parallel
		await notificationsHandler.fetchMissedNotifications()

		verify(await httpClient.fetch(url: any(), method: any(), headers: any(), body: any())).wasCalled(1)
	}
}

extension NotificationsHandler {
	func fetchMissedNotifications() async {
		await withCheckedContinuation { (cont: CheckedContinuation<Void, Never>) in self.fetchMissedNotifications { _ in cont.resume(returning: ()) } }
	}
}
