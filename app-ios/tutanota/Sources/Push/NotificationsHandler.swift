import Foundation

private let MISSED_NOTIFICATION_TTL_SEC: Int64 = 30 * 24 * 60 * 60  // 30 days

/// Downlaods notifications and dispatches them to AlarmManager
class NotificationsHandler {
	private let alarmManager: AlarmManager
	private let notificationStorage: NotificationStorage
	private let urlSession = observableUrlSession()
	private let taskQueue = AsyncQueue()

	init(alarmManager: AlarmManager, notificationStorage: NotificationStorage) {
		self.alarmManager = alarmManager
		self.notificationStorage = notificationStorage
	}

	func initialize() {
		if self.hasNotificationTTLExpired() {
			self.alarmManager.resetStoredState()
		} else {
			// we're scheduling the reschedule before fetching so we don't get
			// two reschedules in parallel
			self.taskQueue.enqueue { [weak self] in self?.alarmManager.rescheduleAlarms() }

			self.fetchMissedNotifications { result in
				switch result {
				case .success: TUTSLog("Successfully processed missed notification")
				case .failure(let err): TUTSLog("Failed to fetch/process missed notification \(err)")
				}
			}
		}
	}

	private func hasNotificationTTLExpired() -> Bool {
		guard let lastMissedNotificationCheckTime = notificationStorage.lastMissedNotificationCheckTime else { return false }
		let sinceNow = lastMissedNotificationCheckTime.timeIntervalSinceNow
		// Important: timeIntervalSinceNow is negative if it's in the past
		return sinceNow < 0 && Int64(abs(sinceNow)) > MISSED_NOTIFICATION_TTL_SEC
	}

	/// Fetch and process missed notification. Will execute fetch operations one by one if it's queued multiple times.  Will wait for suspension if necessary.
	func fetchMissedNotifications(_ completionHandler: @escaping ResponseCallback<Void>) {
		TUTSLog("Adding fetch notification operation to queue")
		self.taskQueue.enqueue { [weak self] in
			let void: Void = ()
			guard let self else {
				completionHandler(.success(void))
				return
			}
			do { try await self.doFetchMissedNotifications() } catch { TUTSLog("Failed to fetch missed notificaiton: \(error)") }
		}
	}

	/// Fetch and process missed notification, actual impl without queuing which makes it easier to just call it recursively.
	private func doFetchMissedNotifications() async throws {
		guard let sseInfo = self.notificationStorage.sseInfo else {
			TUTSLog("No stored SSE info")
			return
		}

		if sseInfo.userIds.isEmpty {
			TUTSLog("No users to download missed notification with")
			self.alarmManager.unscheduleAllAlarms(userId: nil)
			return
		}

		let url = self.missedNotificationUrl(origin: sseInfo.sseOrigin, pushIdentifier: sseInfo.pushIdentifier)

		var request = URLRequest(url: url)
		request.addSysModelHeader()

		let userId: String = sseInfo.userIds[0]
		request.setValue(userId, forHTTPHeaderField: "userIds")

		if let lastNotificationId = self.notificationStorage.lastProcessedNotificationId {
			request.setValue(lastNotificationId, forHTTPHeaderField: "lastProcessedNotificationId")
		}

		TUTSLog("Downloading missed notification with userId \(userId)")

		let (data, response) = try await self.urlSession.data(for: request)
		let httpResponse = response as! HTTPURLResponse
		TUTSLog("Fetched missed notifications with status code \(httpResponse.statusCode)")

		switch HttpStatusCode(rawValue: httpResponse.statusCode) {
		case .notAuthenticated:
			TUTSLog("Not authenticated to download missed notification w/ user \(userId)")
			self.alarmManager.unscheduleAllAlarms(userId: userId)
			self.notificationStorage.removeUser(userId)
			try await self.doFetchMissedNotifications()
		case .serviceUnavailable, .tooManyRequests:
			let suspensionTime = extractSuspensionTime(from: httpResponse)
			TUTSLog("ServiceUnavailable when downloading missed notification, waiting for \(suspensionTime)s")
			try await Task.sleep(nanoseconds: suspensionTime.nanos)
			try await self.doFetchMissedNotifications()
		case .notFound: return
		case .ok:
			self.notificationStorage.lastMissedNotificationCheckTime = Date()
			let missedNotification: MissedNotification
			do { missedNotification = try JSONDecoder().decode(MissedNotification.self, from: data) } catch {
				throw TUTErrorFactory.createError("Failed to parse response for the missed notificaiton, \(error)")
			}
			self.notificationStorage.lastProcessedNotificationId = missedNotification.lastProcessedNotificationId
			try alarmManager.processNewAlarms(missedNotification.alarmNotifications)
		default:
			let errorId = httpResponse.allHeaderFields["Error-Id"]
			let error = NSError(
				domain: TUT_NETWORK_ERROR,
				code: httpResponse.statusCode,
				userInfo: ["message": "Failed to fetch missed notification, error id: \(errorId ?? "")"]
			)
			throw error
		}
	}

	private func missedNotificationUrl(origin: String, pushIdentifier: String) -> URL {
		let base64UrlId = stringToCustomId(customId: pushIdentifier)
		return URL(string: "\(origin)/rest/sys/missednotification/\(base64UrlId)")!
	}
}

private func observableUrlSession() -> URLSession {
	class Metrics: NSObject, URLSessionDataDelegate {
		var requestNum = 0
		func urlSession(_ session: URLSession, task: URLSessionTask, didFinishCollecting metrics: URLSessionTaskMetrics) {
			for metric in metrics.transactionMetrics {
				print("\(requestNum). protocol: \(metric.networkProtocolName!), reused: \(metric.isReusedConnection)")
				requestNum += 1
			}
		}
	}
	let metrics = Metrics()

	let configuration = URLSessionConfiguration.ephemeral
	return URLSession(configuration: configuration, delegate: metrics, delegateQueue: nil)
}
