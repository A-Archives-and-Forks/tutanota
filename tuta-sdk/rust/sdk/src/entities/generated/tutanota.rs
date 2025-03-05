// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Subfiles {
	pub _id: Option<CustomId>,
	pub 27: GeneratedId,
}

impl Entity for Subfiles {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 11,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TutanotaFile {
	pub 21: String,
	pub 22: i64,
	pub 23: Option<String>,
	pub 924: Option<String>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 25: Option<IdTupleGenerated>,
	pub 26: Option<Subfiles>,
	pub 1225: Vec<super::sys::Blob>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for TutanotaFile {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 13,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct FileSystem {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 35: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for FileSystem {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 28,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactMailAddress {
	pub 46: i64,
	pub 47: String,
	pub 48: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactMailAddress {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 44,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactPhoneNumber {
	pub 51: i64,
	pub 52: String,
	pub 53: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactPhoneNumber {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 49,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactAddress {
	pub 56: i64,
	pub 57: String,
	pub 58: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactAddress {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 54,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactSocialId {
	pub 61: i64,
	pub 62: String,
	pub 63: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactSocialId {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 59,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Contact {
	pub 72: String,
	pub 73: String,
	pub 74: String,
	pub 75: String,
	pub 76: Option<DateTime>,
	pub 77: String,
	pub 79: Option<String>,
	pub 849: Option<String>,
	pub 850: Option<String>,
	pub 1083: Option<String>,
	pub 1380: Option<String>,
	pub 1381: Option<String>,
	pub 1382: Option<String>,
	pub 1383: Option<String>,
	pub 1384: Option<String>,
	pub 1385: Option<String>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 80: Vec<ContactMailAddress>,
	pub 81: Vec<ContactPhoneNumber>,
	pub 82: Vec<ContactAddress>,
	pub 83: Vec<ContactSocialId>,
	pub 851: Option<Birthday>,
	pub 852: Option<IdTupleGenerated>,
	pub 1386: Vec<ContactCustomDate>,
	pub 1387: Vec<ContactWebsite>,
	pub 1388: Vec<ContactRelationship>,
	pub 1389: Vec<ContactMessengerHandle>,
	pub 1390: Vec<ContactPronouns>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for Contact {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 64,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ConversationEntry {
	pub 121: String,
	pub 122: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 123: Option<IdTupleGenerated>,
	pub 124: Option<IdTupleGenerated>,
}

impl Entity for ConversationEntry {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 84,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddress {
	pub 94: String,
	pub 95: String,
	pub _id: Option<CustomId>,
	pub 96: Option<IdTupleGenerated>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailAddress {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 92,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Mail {
	pub 105: String,
	pub 107: DateTime,
	pub 108: i64,
	pub 109: bool,
	pub 426: bool,
	pub 466: i64,
	pub 617: Option<String>,
	pub 866: bool,
	pub 896: Option<DateTime>,
	pub 1021: i64,
	pub 1022: Option<i64>,
	pub 1120: i64,
	pub 1307: i64,
	pub 1346: Option<i64>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 111: MailAddress,
	pub 115: Vec<IdTupleGenerated>,
	pub 117: IdTupleGenerated,
	pub 1306: Option<MailAddress>,
	pub 1308: Option<IdTupleGenerated>,
	pub 1309: Option<IdTupleGenerated>,
	pub 1310: Option<super::sys::BucketKey>,
	pub 1465: Vec<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for Mail {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 97,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailBox {
	pub 569: DateTime,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 133: GeneratedId,
	pub 134: GeneratedId,
	pub 443: Option<MailFolderRef>,
	pub 1220: Option<SpamResults>,
	pub 1318: Option<MailDetailsDraftsRef>,
	pub 1463: Vec<MailBag>,
	pub 1464: Option<MailBag>,
	pub 1512: GeneratedId,
	pub 1585: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailBox {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 125,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateExternalUserGroupData {
	pub 141: String,
	#[serde(with = "serde_bytes")]
	pub 142: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 143: Vec<u8>,
	pub 1433: i64,
	pub _id: Option<CustomId>,
}

impl Entity for CreateExternalUserGroupData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 138,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ExternalUserData {
	#[serde(with = "serde_bytes")]
	pub 148: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 149: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 150: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 412: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 669: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 670: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 671: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 672: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 673: Vec<u8>,
	pub 1323: i64,
	pub 1429: i64,
	pub _format: i64,
	pub 151: CreateExternalUserGroupData,
}

impl Entity for ExternalUserData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 145,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactList {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 160: GeneratedId,
	pub 856: Option<PhotosRef>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactList {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 153,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RemoteImapSyncInfo {
	pub 189: bool,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 188: IdTupleGenerated,
}

impl Entity for RemoteImapSyncInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 183,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImapFolder {
	pub 192: String,
	pub 193: String,
	pub 194: String,
	pub _id: Option<CustomId>,
	pub 195: GeneratedId,
}

impl Entity for ImapFolder {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 190,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImapSyncState {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 201: Vec<ImapFolder>,
}

impl Entity for ImapSyncState {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 196,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImapSyncConfiguration {
	pub 211: String,
	pub 212: i64,
	pub 213: String,
	pub 214: String,
	pub _id: Option<CustomId>,
	pub 215: Option<GeneratedId>,
}

impl Entity for ImapSyncConfiguration {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 209,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TutanotaProperties {
	#[serde(with = "serde_bytes")]
	pub 410: Option<Vec<u8>>,
	pub 418: Option<String>,
	pub 469: Option<String>,
	pub 470: bool,
	pub 471: String,
	pub 472: i64,
	pub 568: bool,
	pub 676: bool,
	pub 897: i64,
	pub 1434: Option<i64>,
	pub 1510: bool,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 221: Option<IdTupleGenerated>,
	pub 222: Vec<ImapSyncConfiguration>,
	pub 578: Vec<InboxRule>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for TutanotaProperties {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 216,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NotificationMail {
	pub 225: String,
	pub 226: String,
	pub 227: String,
	pub 228: String,
	pub 417: String,
	pub _id: Option<CustomId>,
}

impl Entity for NotificationMail {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 223,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DeleteMailData {
	pub _format: i64,
	pub 421: Vec<IdTupleGenerated>,
	pub 724: Option<IdTupleGenerated>,
}

impl Entity for DeleteMailData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 419,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailFolder {
	pub 435: String,
	pub 436: i64,
	pub 1479: Option<String>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 439: Option<IdTupleGenerated>,
	pub 1459: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailFolder {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 429,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailFolderRef {
	pub _id: Option<CustomId>,
	pub 442: GeneratedId,
}

impl Entity for MailFolderRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 440,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MoveMailData {
	pub _format: i64,
	pub 447: IdTupleGenerated,
	pub 448: Vec<IdTupleGenerated>,
	pub 1466: IdTupleGenerated,
}

impl Entity for MoveMailData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 445,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateMailFolderData {
	pub 453: String,
	#[serde(with = "serde_bytes")]
	pub 454: Vec<u8>,
	pub 1268: Option<GeneratedId>,
	pub 1414: i64,
	pub _format: i64,
	pub 452: Option<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CreateMailFolderData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 450,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateMailFolderReturn {
	pub _format: i64,
	pub 457: IdTupleGenerated,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CreateMailFolderReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 455,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DeleteMailFolderData {
	pub _format: i64,
	pub 460: Vec<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DeleteMailFolderData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 458,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EncryptTutanotaPropertiesData {
	#[serde(with = "serde_bytes")]
	pub 476: Vec<u8>,
	pub 1428: i64,
	pub _format: i64,
	pub 475: GeneratedId,
}

impl Entity for EncryptTutanotaPropertiesData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 473,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftRecipient {
	pub 484: String,
	pub 485: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DraftRecipient {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 482,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NewDraftAttachment {
	#[serde(with = "serde_bytes")]
	pub 488: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 489: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 925: Option<Vec<u8>>,
	pub _id: Option<CustomId>,
	pub 1226: Vec<super::sys::BlobReferenceTokenWrapper>,
}

impl Entity for NewDraftAttachment {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 486,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftAttachment {
	#[serde(with = "serde_bytes")]
	pub 493: Vec<u8>,
	pub 1430: i64,
	pub _id: Option<CustomId>,
	pub 494: Option<NewDraftAttachment>,
	pub 495: Option<IdTupleGenerated>,
}

impl Entity for DraftAttachment {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 491,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftData {
	pub 498: String,
	pub 499: String,
	pub 500: String,
	pub 501: String,
	pub 502: bool,
	pub 1116: i64,
	pub 1194: Option<String>,
	pub _id: Option<CustomId>,
	pub 503: Vec<DraftRecipient>,
	pub 504: Vec<DraftRecipient>,
	pub 505: Vec<DraftRecipient>,
	pub 506: Vec<DraftAttachment>,
	pub 507: Vec<IdTupleGenerated>,
	pub 819: Vec<EncryptedMailAddress>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DraftData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 496,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftCreateData {
	pub 510: Option<String>,
	pub 511: i64,
	#[serde(with = "serde_bytes")]
	pub 512: Vec<u8>,
	pub 1427: i64,
	pub _format: i64,
	pub 515: DraftData,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DraftCreateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 508,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftCreateReturn {
	pub _format: i64,
	pub 518: IdTupleGenerated,
}

impl Entity for DraftCreateReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 516,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftUpdateData {
	pub _format: i64,
	pub 521: DraftData,
	pub 522: IdTupleGenerated,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DraftUpdateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 519,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DraftUpdateReturn {
	pub _format: i64,
	pub 525: Vec<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DraftUpdateReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 523,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InternalRecipientKeyData {
	pub 529: String,
	#[serde(with = "serde_bytes")]
	pub 530: Vec<u8>,
	pub 531: i64,
	pub 1352: i64,
	pub 1431: Option<i64>,
	pub _id: Option<CustomId>,
}

impl Entity for InternalRecipientKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 527,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecureExternalRecipientKeyData {
	pub 534: String,
	#[serde(with = "serde_bytes")]
	pub 536: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 538: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 539: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 540: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 599: Vec<u8>,
	pub 1324: i64,
	pub 1417: i64,
	pub 1445: i64,
	pub _id: Option<CustomId>,
}

impl Entity for SecureExternalRecipientKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 532,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AttachmentKeyData {
	#[serde(with = "serde_bytes")]
	pub 544: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 545: Option<Vec<u8>>,
	pub _id: Option<CustomId>,
	pub 546: IdTupleGenerated,
}

impl Entity for AttachmentKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 542,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SendDraftData {
	pub 549: String,
	#[serde(with = "serde_bytes")]
	pub 550: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 551: Option<Vec<u8>>,
	pub 552: Option<String>,
	pub 675: bool,
	pub 1117: bool,
	#[serde(with = "serde_bytes")]
	pub 1444: Option<Vec<u8>>,
	pub _format: i64,
	pub 553: Vec<InternalRecipientKeyData>,
	pub 554: Vec<SecureExternalRecipientKeyData>,
	pub 555: Vec<AttachmentKeyData>,
	pub 556: IdTupleGenerated,
	pub 1353: Vec<SymEncInternalRecipientKeyData>,
}

impl Entity for SendDraftData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 547,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SendDraftReturn {
	pub 559: String,
	pub 560: DateTime,
	pub _format: i64,
	pub 561: Vec<NotificationMail>,
	pub 562: IdTupleGenerated,
}

impl Entity for SendDraftReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 557,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReceiveInfoServiceData {
	pub 1121: String,
	pub _format: i64,
}

impl Entity for ReceiveInfoServiceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 570,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InboxRule {
	pub 575: String,
	pub 576: String,
	pub _id: Option<CustomId>,
	pub 577: IdTupleGenerated,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for InboxRule {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 573,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EncryptedMailAddress {
	pub 614: String,
	pub 615: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for EncryptedMailAddress {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 612,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAccountUserData {
	pub 624: String,
	#[serde(with = "serde_bytes")]
	pub 625: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 626: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 627: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 629: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 630: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 631: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 632: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 633: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 634: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 635: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 636: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 637: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 638: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 639: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 640: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 641: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 892: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 893: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 894: Vec<u8>,
	pub 1322: i64,
	pub 1426: i64,
	pub _id: Option<CustomId>,
}

impl Entity for UserAccountUserData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 622,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InternalGroupData {
	#[serde(with = "serde_bytes")]
	pub 644: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 645: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 646: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 647: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1342: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1343: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1344: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1345: Option<Vec<u8>>,
	pub 1415: i64,
	pub 1416: i64,
	pub _id: Option<CustomId>,
	pub 874: Option<GeneratedId>,
}

impl Entity for InternalGroupData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 642,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerAccountCreateData {
	pub 650: String,
	pub 651: Option<DateTime>,
	pub 652: String,
	#[serde(with = "serde_bytes")]
	pub 654: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 655: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 659: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 660: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 661: Vec<u8>,
	pub 873: String,
	pub 1355: i64,
	pub 1421: i64,
	pub 1422: i64,
	pub 1511: i64,
	pub _format: i64,
	pub 653: UserAccountUserData,
	pub 656: InternalGroupData,
	pub 657: InternalGroupData,
	pub 658: InternalGroupData,
}

impl Entity for CustomerAccountCreateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 648,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAccountCreateData {
	pub 665: Option<DateTime>,
	pub _format: i64,
	pub 666: UserAccountUserData,
	pub 667: InternalGroupData,
}

impl Entity for UserAccountCreateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 663,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailboxServerProperties {
	pub 683: bool,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for MailboxServerProperties {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 677,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailboxGroupRoot {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 699: GeneratedId,
	pub 700: GeneratedId,
	pub 1119: Option<CalendarEventUpdateList>,
	pub 1150: Option<GeneratedId>,
	pub 1151: Option<OutOfOfficeNotificationRecipientList>,
	pub 1203: Option<GeneratedId>,
}

impl Entity for MailboxGroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 693,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateMailGroupData {
	pub 709: String,
	#[serde(with = "serde_bytes")]
	pub 710: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 711: Vec<u8>,
	pub _format: i64,
	pub 712: InternalGroupData,
}

impl Entity for CreateMailGroupData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 707,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DeleteGroupData {
	pub 715: bool,
	pub _format: i64,
	pub 716: GeneratedId,
}

impl Entity for DeleteGroupData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 713,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Birthday {
	pub 846: i64,
	pub 847: i64,
	pub 848: Option<i64>,
	pub _id: Option<CustomId>,
}

impl Entity for Birthday {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 844,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PhotosRef {
	pub _id: Option<CustomId>,
	pub 855: GeneratedId,
}

impl Entity for PhotosRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 853,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ListUnsubscribeData {
	pub 870: String,
	pub 871: String,
	pub _format: i64,
	pub 869: IdTupleGenerated,
}

impl Entity for ListUnsubscribeData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 867,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarRepeatRule {
	pub 928: i64,
	pub 929: i64,
	pub 930: Option<i64>,
	pub 931: i64,
	pub 932: String,
	pub _id: Option<CustomId>,
	pub 1319: Vec<super::sys::DateWrapper>,
	pub 1590: Vec<AdvancedRepeatRule>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CalendarRepeatRule {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 926,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEvent {
	pub 940: String,
	pub 941: String,
	pub 942: DateTime,
	pub 943: DateTime,
	pub 944: String,
	pub 988: Option<String>,
	#[serde(with = "serde_bytes")]
	pub 1088: Option<Vec<u8>>,
	pub 1089: i64,
	pub 1090: Option<bool>,
	pub 1320: Option<DateTime>,
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 945: Option<CalendarRepeatRule>,
	pub 946: Vec<IdTupleGenerated>,
	pub 1091: Vec<CalendarEventAttendee>,
	pub 1092: Option<EncryptedMailAddress>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CalendarEvent {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 933,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarGroupRoot {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 954: GeneratedId,
	pub 955: GeneratedId,
	pub 1103: Option<CalendarEventIndexRef>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CalendarGroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 947,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAreaGroupData {
	#[serde(with = "serde_bytes")]
	pub 958: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 959: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 960: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 961: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 962: Vec<u8>,
	pub 1423: Option<i64>,
	pub 1424: i64,
	pub 1425: i64,
	pub _id: Option<CustomId>,
	pub 963: Option<GeneratedId>,
}

impl Entity for UserAreaGroupData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 956,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAreaGroupPostData {
	pub _format: i64,
	pub 966: UserAreaGroupData,
}

impl Entity for UserAreaGroupPostData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 964,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupSettings {
	pub 971: String,
	pub 1020: Option<String>,
	pub 1468: Option<String>,
	pub _id: Option<CustomId>,
	pub 970: GeneratedId,
	pub 1449: Vec<DefaultAlarmInfo>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for GroupSettings {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 968,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserSettingsGroupRoot {
	pub 980: i64,
	pub 981: i64,
	pub 1234: Option<bool>,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 979: Vec<GroupSettings>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for UserSettingsGroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 972,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarDeleteData {
	pub _format: i64,
	pub 984: GeneratedId,
}

impl Entity for CalendarDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 982,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateGroupPostReturn {
	pub _format: i64,
	pub 987: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CreateGroupPostReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 985,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SharedGroupData {
	pub 994: i64,
	#[serde(with = "serde_bytes")]
	pub 995: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 996: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 997: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 998: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 999: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1000: Vec<u8>,
	pub 1001: GeneratedId,
	pub 1420: i64,
	pub _id: Option<CustomId>,
}

impl Entity for SharedGroupData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 992,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupInvitationPostData {
	pub _format: i64,
	pub 1004: SharedGroupData,
	pub 1005: Vec<InternalRecipientKeyData>,
}

impl Entity for GroupInvitationPostData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1002,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupInvitationPostReturn {
	pub _format: i64,
	pub 1008: Vec<MailAddress>,
	pub 1009: Vec<MailAddress>,
	pub 1010: Vec<MailAddress>,
}

impl Entity for GroupInvitationPostReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1006,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupInvitationPutData {
	#[serde(with = "serde_bytes")]
	pub 1013: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1014: Vec<u8>,
	pub 1418: i64,
	pub 1419: i64,
	pub _format: i64,
	pub 1015: IdTupleGenerated,
}

impl Entity for GroupInvitationPutData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1011,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupInvitationDeleteData {
	pub _format: i64,
	pub 1018: IdTupleGenerated,
}

impl Entity for GroupInvitationDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1016,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReportedMailFieldMarker {
	pub 1025: String,
	pub 1026: i64,
	pub _id: Option<CustomId>,
}

impl Entity for ReportedMailFieldMarker {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1023,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PhishingMarkerWebsocketData {
	pub 1036: GeneratedId,
	pub _format: i64,
	pub 1037: Vec<ReportedMailFieldMarker>,
}

impl Entity for PhishingMarkerWebsocketData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1034,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReportMailPostData {
	#[serde(with = "serde_bytes")]
	pub 1068: Vec<u8>,
	pub 1082: i64,
	pub _format: i64,
	pub 1069: IdTupleGenerated,
}

impl Entity for ReportMailPostData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1066,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEventAttendee {
	pub 1086: i64,
	pub _id: Option<CustomId>,
	pub 1087: EncryptedMailAddress,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CalendarEventAttendee {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1084,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEventUidIndex {
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1099: Option<IdTupleCustom>,
	pub 1321: Vec<IdTupleCustom>,
}

impl Entity for CalendarEventUidIndex {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1093,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEventIndexRef {
	pub _id: Option<CustomId>,
	pub 1102: GeneratedId,
}

impl Entity for CalendarEventIndexRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1100,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEventUpdate {
	pub 1111: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1112: IdTupleGenerated,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CalendarEventUpdate {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1104,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEventUpdateList {
	pub _id: Option<CustomId>,
	pub 1115: GeneratedId,
}

impl Entity for CalendarEventUpdateList {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1113,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EntropyData {
	#[serde(with = "serde_bytes")]
	pub 1124: Vec<u8>,
	pub 1432: i64,
	pub _format: i64,
}

impl Entity for EntropyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1122,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct OutOfOfficeNotificationMessage {
	pub 1128: String,
	pub 1129: String,
	pub 1130: i64,
	pub _id: Option<CustomId>,
}

impl Entity for OutOfOfficeNotificationMessage {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1126,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct OutOfOfficeNotification {
	pub 1137: bool,
	pub 1138: Option<DateTime>,
	pub 1139: Option<DateTime>,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1140: Vec<OutOfOfficeNotificationMessage>,
}

impl Entity for OutOfOfficeNotification {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1131,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct OutOfOfficeNotificationRecipientList {
	pub _id: Option<CustomId>,
	pub 1149: GeneratedId,
}

impl Entity for OutOfOfficeNotificationRecipientList {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1147,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EmailTemplateContent {
	pub 1156: String,
	pub 1157: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for EmailTemplateContent {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1154,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EmailTemplate {
	pub 1165: String,
	pub 1166: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1167: Vec<EmailTemplateContent>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for EmailTemplate {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1158,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct KnowledgeBaseEntryKeyword {
	pub 1170: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for KnowledgeBaseEntryKeyword {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1168,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct KnowledgeBaseEntry {
	pub 1178: String,
	pub 1179: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1180: Vec<KnowledgeBaseEntryKeyword>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for KnowledgeBaseEntry {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1171,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TemplateGroupRoot {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1188: GeneratedId,
	pub 1189: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for TemplateGroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1181,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAreaGroupDeleteData {
	pub _format: i64,
	pub 1192: GeneratedId,
}

impl Entity for UserAreaGroupDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1190,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailboxProperties {
	pub 1202: i64,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1267: Vec<MailAddressProperties>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailboxProperties {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1195,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SpamResults {
	pub _id: Option<CustomId>,
	pub 1219: GeneratedId,
}

impl Entity for SpamResults {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1217,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NewsId {
	pub 1247: String,
	pub 1248: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for NewsId {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1245,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NewsOut {
	pub _format: i64,
	pub 1258: Vec<NewsId>,
}

impl Entity for NewsOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1256,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NewsIn {
	pub 1261: Option<GeneratedId>,
	pub _format: i64,
}

impl Entity for NewsIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1259,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressProperties {
	pub 1265: String,
	pub 1266: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailAddressProperties {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1263,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Header {
	pub 1271: Option<String>,
	pub 1272: Option<String>,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for Header {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1269,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Body {
	pub 1275: Option<String>,
	pub 1276: Option<String>,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for Body {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1273,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Recipients {
	pub _id: Option<CustomId>,
	pub 1279: Vec<MailAddress>,
	pub 1280: Vec<MailAddress>,
	pub 1281: Vec<MailAddress>,
}

impl Entity for Recipients {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1277,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailDetails {
	pub 1284: DateTime,
	pub 1289: i64,
	pub _id: Option<CustomId>,
	pub 1285: Vec<EncryptedMailAddress>,
	pub 1286: Recipients,
	pub 1287: Option<Header>,
	pub 1288: Body,
}

impl Entity for MailDetails {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1282,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailDetailsDraft {
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1297: MailDetails,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailDetailsDraft {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1290,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailDetailsBlob {
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1305: MailDetails,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MailDetailsBlob {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1298,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UpdateMailFolderData {
	pub _format: i64,
	pub 1313: IdTupleGenerated,
	pub 1314: Option<IdTupleGenerated>,
}

impl Entity for UpdateMailFolderData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1311,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailDetailsDraftsRef {
	pub _id: Option<CustomId>,
	pub 1317: GeneratedId,
}

impl Entity for MailDetailsDraftsRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1315,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactListEntry {
	pub 1332: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactListEntry {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1325,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactListGroupRoot {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1340: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactListGroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1333,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SymEncInternalRecipientKeyData {
	pub 1349: String,
	#[serde(with = "serde_bytes")]
	pub 1350: Vec<u8>,
	pub 1435: i64,
	pub _id: Option<CustomId>,
	pub 1351: GeneratedId,
}

impl Entity for SymEncInternalRecipientKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1347,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactCustomDate {
	pub 1358: i64,
	pub 1359: String,
	pub 1360: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactCustomDate {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1356,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactWebsite {
	pub 1363: i64,
	pub 1364: String,
	pub 1365: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactWebsite {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1361,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactRelationship {
	pub 1368: i64,
	pub 1369: String,
	pub 1370: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactRelationship {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1366,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactMessengerHandle {
	pub 1373: i64,
	pub 1374: String,
	pub 1375: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactMessengerHandle {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1371,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ContactPronouns {
	pub 1378: String,
	pub 1379: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ContactPronouns {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1376,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TranslationGetIn {
	pub 1438: String,
	pub _format: i64,
}

impl Entity for TranslationGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1436,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TranslationGetOut {
	pub 1441: String,
	pub 1442: String,
	pub _format: i64,
}

impl Entity for TranslationGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1439,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DefaultAlarmInfo {
	pub 1448: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DefaultAlarmInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1446,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailSetEntry {
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1456: IdTupleGenerated,
}

impl Entity for MailSetEntry {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1450,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailBag {
	pub _id: Option<CustomId>,
	pub 1462: GeneratedId,
}

impl Entity for MailBag {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1460,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SimpleMoveMailPostIn {
	pub 1472: i64,
	pub _format: i64,
	pub 1471: Vec<IdTupleGenerated>,
}

impl Entity for SimpleMoveMailPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1469,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UnreadMailStatePostIn {
	pub 1477: bool,
	pub _format: i64,
	pub 1476: Vec<IdTupleGenerated>,
}

impl Entity for UnreadMailStatePostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1474,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ManageLabelServiceLabelData {
	pub 1482: String,
	pub 1483: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ManageLabelServiceLabelData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1480,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ManageLabelServicePostIn {
	#[serde(with = "serde_bytes")]
	pub 1486: Vec<u8>,
	pub 1487: i64,
	pub 1488: GeneratedId,
	pub _format: i64,
	pub 1489: ManageLabelServiceLabelData,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ManageLabelServicePostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1484,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ManageLabelServiceDeleteIn {
	pub _format: i64,
	pub 1502: IdTupleGenerated,
}

impl Entity for ManageLabelServiceDeleteIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1500,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ApplyLabelServicePostIn {
	pub _format: i64,
	pub 1506: Vec<IdTupleGenerated>,
	pub 1507: Vec<IdTupleGenerated>,
	pub 1508: Vec<IdTupleGenerated>,
}

impl Entity for ApplyLabelServicePostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1504,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailDataMailReference {
	pub 1515: String,
	pub _id: Option<CustomId>,
}

impl Entity for ImportMailDataMailReference {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1513,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NewImportAttachment {
	#[serde(with = "serde_bytes")]
	pub 1518: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1519: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1520: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1521: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1522: Option<Vec<u8>>,
	pub _id: Option<CustomId>,
	pub 1523: Vec<super::sys::BlobReferenceTokenWrapper>,
}

impl Entity for NewImportAttachment {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1516,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportAttachment {
	#[serde(with = "serde_bytes")]
	pub 1526: Vec<u8>,
	pub 1527: i64,
	pub _id: Option<CustomId>,
	pub 1528: Option<NewImportAttachment>,
	pub 1529: Option<IdTupleGenerated>,
}

impl Entity for ImportAttachment {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1524,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailData {
	#[serde(with = "serde_bytes")]
	pub 1532: Vec<u8>,
	pub 1533: i64,
	pub 1534: String,
	pub 1535: String,
	pub 1536: DateTime,
	pub 1537: i64,
	pub 1538: bool,
	pub 1539: Option<String>,
	pub 1540: Option<String>,
	pub 1541: bool,
	pub 1542: i64,
	pub 1543: i64,
	pub 1544: Option<String>,
	pub 1545: i64,
	pub 1546: String,
	pub _format: i64,
	pub 1547: Vec<ImportMailDataMailReference>,
	pub 1548: MailAddress,
	pub 1549: Vec<EncryptedMailAddress>,
	pub 1550: Recipients,
	pub 1551: Vec<ImportAttachment>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ImportMailData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1530,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportedMail {
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1558: IdTupleCustom,
}

impl Entity for ImportedMail {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1552,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailState {
	pub 1565: i64,
	pub 1566: i64,
	pub 1567: i64,
	pub 1600: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1568: GeneratedId,
	pub 1569: IdTupleGenerated,
}

impl Entity for ImportMailState {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1559,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailPostIn {
	pub _format: i64,
	pub 1577: IdTupleGenerated,
	pub 1578: Vec<super::sys::StringWrapper>,
}

impl Entity for ImportMailPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1570,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailPostOut {
	pub _format: i64,
}

impl Entity for ImportMailPostOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1579,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailGetIn {
	pub 1594: GeneratedId,
	pub 1595: i64,
	#[serde(with = "serde_bytes")]
	pub 1596: Vec<u8>,
	pub 1597: String,
	pub 1598: i64,
	pub _format: i64,
	pub 1599: IdTupleGenerated,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ImportMailGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1582,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AdvancedRepeatRule {
	pub 1588: i64,
	pub 1589: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for AdvancedRepeatRule {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1586,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ImportMailGetOut {
	pub _format: i64,
	pub 1593: IdTupleGenerated,
}

impl Entity for ImportMailGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1591,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailExportTokenServicePostOut {
	pub 1607: String,
	pub _format: i64,
}

impl Entity for MailExportTokenServicePostOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1605,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SupportTopic {
	pub 1620: DateTime,
	pub 1621: String,
	pub 1622: String,
	pub 1623: String,
	pub 1624: String,
	pub 1625: i64,
	pub _id: Option<CustomId>,
}

impl Entity for SupportTopic {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1618,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SupportCategory {
	pub 1628: String,
	pub 1629: String,
	pub 1630: String,
	pub 1631: String,
	pub 1632: String,
	pub _id: Option<CustomId>,
	pub 1633: Vec<SupportTopic>,
}

impl Entity for SupportCategory {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1626,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SupportData {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1640: Vec<SupportCategory>,
}

impl Entity for SupportData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "tutanota",
			type_id: 1634,
		}
	}
}
