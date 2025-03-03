import { OfflineMigration } from "../OfflineStorageMigrator.js"
import { OfflineStorage } from "../OfflineStorage.js"

export const sys123: OfflineMigration = {
	app: "sys",
	version: 123,
	async migrate(storage: OfflineStorage) {},
}
