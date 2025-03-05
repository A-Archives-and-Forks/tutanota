// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct KeyPair {
	#[serde(with = "serde_bytes")]
	pub 2: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 3: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2144: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2145: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2146: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2147: Option<Vec<u8>>,
	pub _id: Option<CustomId>,
}

impl Entity for KeyPair {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 0,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Group {
	pub 10: i64,
	#[serde(with = "serde_bytes")]
	pub 11: Option<Vec<u8>>,
	pub 12: bool,
	pub 982: bool,
	pub 2270: Option<i64>,
	pub 2271: i64,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 13: Option<KeyPair>,
	pub 224: Option<GeneratedId>,
	pub 225: Option<GeneratedId>,
	pub 226: Option<GeneratedId>,
	pub 227: IdTupleGenerated,
	pub 228: GeneratedId,
	pub 229: GeneratedId,
	pub 1881: Vec<ArchiveType>,
	pub 2092: Option<GeneratedId>,
	pub 2273: Option<GroupKeysRef>,
	pub 2475: Option<PubEncKeyData>,
}

impl Entity for Group {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 5,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupInfo {
	pub 21: String,
	pub 22: Option<String>,
	pub 23: DateTime,
	pub 24: Option<DateTime>,
	pub 1286: Option<i64>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _listEncSessionKey: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 20: GeneratedId,
	pub 687: Vec<MailAddressAlias>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for GroupInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 14,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupMembership {
	#[serde(with = "serde_bytes")]
	pub 27: Vec<u8>,
	pub 28: bool,
	pub 1030: Option<i64>,
	pub 1626: Option<i64>,
	pub 2246: i64,
	pub 2247: i64,
	pub _id: Option<CustomId>,
	pub 29: GeneratedId,
	pub 30: IdTupleGenerated,
	pub 230: IdTupleGenerated,
}

impl Entity for GroupMembership {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 25,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Customer {
	pub 36: i64,
	pub 926: i64,
	pub 1347: bool,
	pub 1754: bool,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 37: GeneratedId,
	pub 38: GeneratedId,
	pub 39: GeneratedId,
	pub 40: GeneratedId,
	pub 41: GeneratedId,
	pub 42: GeneratedId,
	pub 160: IdTupleGenerated,
	pub 662: Option<GeneratedId>,
	pub 960: Option<GeneratedId>,
	pub 992: Option<UserAreaGroups>,
	pub 1161: Option<AuditLogRef>,
	pub 1256: Vec<Feature>,
	pub 1276: Option<WhitelabelParent>,
	pub 1277: Option<WhitelabelChildrenRef>,
	pub 1348: Option<IdTupleGenerated>,
	pub 1750: Option<RejectedSendersRef>,
	pub 2061: Option<GeneratedId>,
}

impl Entity for Customer {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 31,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AuthenticatedDevice {
	pub 45: i64,
	pub 46: String,
	#[serde(with = "serde_bytes")]
	pub 47: Vec<u8>,
	pub _id: Option<CustomId>,
}

impl Entity for AuthenticatedDevice {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 43,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Login {
	pub 53: DateTime,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for Login {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 48,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactorAuthentication {
	pub 59: String,
	pub 60: i64,
	pub 61: bool,
	pub 62: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for SecondFactorAuthentication {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 54,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VariableExternalAuthInfo {
	#[serde(with = "serde_bytes")]
	pub 71: Option<Vec<u8>>,
	pub 72: Option<DateTime>,
	#[serde(with = "serde_bytes")]
	pub 73: Option<Vec<u8>>,
	pub 74: i64,
	pub 75: DateTime,
	pub 76: i64,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for VariableExternalAuthInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 66,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserExternalAuthInfo {
	pub 79: GeneratedId,
	#[serde(with = "serde_bytes")]
	pub 80: Option<Vec<u8>>,
	pub 81: Option<String>,
	pub 82: i64,
	pub _id: Option<CustomId>,
	pub 83: GeneratedId,
}

impl Entity for UserExternalAuthInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 77,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct User {
	#[serde(with = "serde_bytes")]
	pub 90: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 91: Vec<u8>,
	pub 92: i64,
	pub 93: bool,
	pub 1117: bool,
	pub 2132: i64,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 95: GroupMembership,
	pub 96: Vec<GroupMembership>,
	pub 97: Vec<AuthenticatedDevice>,
	pub 98: Option<UserExternalAuthInfo>,
	pub 99: Option<GeneratedId>,
	pub 100: GeneratedId,
	pub 101: GeneratedId,
	pub 102: GeneratedId,
	pub 638: Option<PushIdentifierList>,
	pub 1210: Option<UserAuthentication>,
	pub 1552: Option<UserAlarmInfoListType>,
}

impl Entity for User {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 84,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ExternalUserReference {
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 108: GeneratedId,
	pub 109: GeneratedId,
}

impl Entity for ExternalUserReference {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 103,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupRoot {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 116: GeneratedId,
	pub 117: GeneratedId,
	pub 999: Option<UserAreaGroups>,
}

impl Entity for GroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 110,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BucketPermission {
	pub 123: i64,
	#[serde(with = "serde_bytes")]
	pub 124: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 125: Option<Vec<u8>>,
	pub 126: Option<i64>,
	#[serde(with = "serde_bytes")]
	pub 1001: Option<Vec<u8>>,
	pub 2157: i64,
	pub 2248: Option<i64>,
	pub 2249: Option<i64>,
	pub 2250: Option<i64>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 128: GeneratedId,
}

impl Entity for BucketPermission {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 118,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Bucket {
	pub _id: Option<CustomId>,
	pub 131: GeneratedId,
}

impl Entity for Bucket {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 129,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Permission {
	pub 137: i64,
	#[serde(with = "serde_bytes")]
	pub 138: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 139: Option<Vec<u8>>,
	pub 140: Option<String>,
	pub 1523: Option<i64>,
	pub 1524: Option<String>,
	pub 2251: Option<i64>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 141: Option<GeneratedId>,
	pub 142: Option<Bucket>,
}

impl Entity for Permission {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 132,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AccountingInfo {
	pub 592: Option<DateTime>,
	pub 593: i64,
	pub 762: String,
	pub 763: String,
	pub 764: Option<String>,
	pub 765: i64,
	pub 766: String,
	pub 767: Option<i64>,
	pub 768: Option<String>,
	pub 769: i64,
	pub 770: Option<String>,
	pub 1060: Option<String>,
	pub 1312: Option<String>,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _modified: DateTime,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 771: Option<GeneratedId>,
	pub 2424: Option<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for AccountingInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 143,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerInfo {
	pub 153: Option<String>,
	pub 154: String,
	pub 155: DateTime,
	pub 156: Option<DateTime>,
	pub 157: Option<DateTime>,
	pub 597: String,
	pub 639: Option<DateTime>,
	pub 640: Option<String>,
	pub 650: i64,
	pub 725: String,
	pub 976: i64,
	pub 977: i64,
	pub 1067: i64,
	pub 1068: i64,
	pub 1381: bool,
	pub 2093: i64,
	pub 2094: i64,
	pub 2098: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 158: GeneratedId,
	pub 159: GeneratedId,
	pub 726: Vec<DomainInfo>,
	pub 727: Option<BookingsRef>,
	pub 1076: Option<GeneratedId>,
	pub 1794: Option<GiftCardsRef>,
	pub 2014: Option<IdTupleGenerated>,
	pub 2072: Option<GeneratedId>,
	pub 2114: Option<PlanConfiguration>,
	pub 2197: Option<GeneratedId>,
}

impl Entity for CustomerInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 148,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SentGroupInvitation {
	pub 1600: String,
	pub 1601: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 203: GeneratedId,
	pub 1617: Option<IdTupleGenerated>,
}

impl Entity for SentGroupInvitation {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 195,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressToGroup {
	pub _format: i64,
	pub _id: Option<CustomId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 209: Option<GeneratedId>,
}

impl Entity for MailAddressToGroup {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 204,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupMember {
	pub 1625: Option<i64>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 221: IdTupleGenerated,
	pub 222: GeneratedId,
	pub 223: GeneratedId,
}

impl Entity for GroupMember {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 216,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RootInstance {
	pub 236: GeneratedId,
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for RootInstance {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 231,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VersionInfo {
	pub 242: String,
	pub 243: i64,
	pub 244: Option<GeneratedId>,
	pub 245: DateTime,
	pub 246: String,
	#[serde(with = "serde_bytes")]
	pub 247: Option<Vec<u8>>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 248: GeneratedId,
	pub 249: IdTupleGenerated,
}

impl Entity for VersionInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 237,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SystemKeysReturn {
	#[serde(with = "serde_bytes")]
	pub 303: Option<Vec<u8>>,
	pub 304: i64,
	#[serde(with = "serde_bytes")]
	pub 305: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 306: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2155: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2156: Option<Vec<u8>>,
	pub 2278: i64,
	pub 2279: i64,
	pub _format: i64,
	pub 880: Option<GeneratedId>,
	pub 881: Option<GeneratedId>,
}

impl Entity for SystemKeysReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 301,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RegistrationServiceData {
	pub 325: i64,
	pub 874: Option<String>,
	pub _format: i64,
}

impl Entity for RegistrationServiceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 316,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RegistrationReturn {
	pub 328: String,
	pub _format: i64,
}

impl Entity for RegistrationReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 326,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SendRegistrationCodeData {
	pub 343: String,
	pub 344: String,
	pub 345: i64,
	pub 346: String,
	pub _format: i64,
}

impl Entity for SendRegistrationCodeData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 341,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SendRegistrationCodeReturn {
	pub 349: String,
	pub _format: i64,
}

impl Entity for SendRegistrationCodeReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 347,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VerifyRegistrationCodeData {
	pub 353: String,
	pub 354: String,
	pub _format: i64,
}

impl Entity for VerifyRegistrationCodeData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 351,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserDataDelete {
	pub 406: bool,
	pub 879: Option<DateTime>,
	pub _format: i64,
	pub 407: GeneratedId,
}

impl Entity for UserDataDelete {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 404,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PublicKeyGetIn {
	pub 411: String,
	pub 2244: Option<i64>,
	pub 2468: i64,
	pub _format: i64,
}

impl Entity for PublicKeyGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 409,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PublicKeyGetOut {
	#[serde(with = "serde_bytes")]
	pub 414: Option<Vec<u8>>,
	pub 415: i64,
	#[serde(with = "serde_bytes")]
	pub 2148: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2149: Option<Vec<u8>>,
	pub _format: i64,
}

impl Entity for PublicKeyGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 412,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SaltData {
	pub 419: String,
	pub _format: i64,
}

impl Entity for SaltData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 417,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SaltReturn {
	#[serde(with = "serde_bytes")]
	pub 422: Vec<u8>,
	pub 2133: i64,
	pub _format: i64,
}

impl Entity for SaltReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 420,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AutoLoginDataGet {
	pub 434: String,
	pub _format: i64,
	pub 433: GeneratedId,
}

impl Entity for AutoLoginDataGet {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 431,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AutoLoginDataDelete {
	pub 437: String,
	pub _format: i64,
}

impl Entity for AutoLoginDataDelete {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 435,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AutoLoginDataReturn {
	#[serde(with = "serde_bytes")]
	pub 440: Vec<u8>,
	pub _format: i64,
}

impl Entity for AutoLoginDataReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 438,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AutoLoginPostReturn {
	pub 443: String,
	pub _format: i64,
}

impl Entity for AutoLoginPostReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 441,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UpdatePermissionKeyData {
	#[serde(with = "serde_bytes")]
	pub 1031: Vec<u8>,
	pub 2245: i64,
	pub _format: i64,
	pub 450: IdTupleGenerated,
	pub 451: IdTupleGenerated,
}

impl Entity for UpdatePermissionKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 445,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Authentication {
	pub 456: Option<String>,
	pub 968: Option<String>,
	pub 1239: Option<String>,
	pub _id: Option<CustomId>,
	pub 455: GeneratedId,
}

impl Entity for Authentication {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 453,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Chat {
	pub 459: GeneratedId,
	pub 460: GeneratedId,
	pub 461: String,
	pub _id: Option<CustomId>,
}

impl Entity for Chat {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 457,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EntityUpdate {
	pub 464: String,
	pub 465: String,
	pub 466: String,
	pub 467: String,
	pub 624: i64,
	pub 2554: i64,
	pub _id: Option<CustomId>,
}

impl Entity for EntityUpdate {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 462,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SysException {
	pub 470: String,
	pub 471: String,
	pub _id: Option<CustomId>,
}

impl Entity for SysException {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 468,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Version {
	pub 482: GeneratedId,
	pub 483: DateTime,
	pub 484: String,
	pub _id: Option<CustomId>,
	pub 485: GeneratedId,
	pub 486: IdTupleGenerated,
}

impl Entity for Version {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 480,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VersionData {
	pub 489: String,
	pub 490: i64,
	pub 491: GeneratedId,
	pub 492: Option<GeneratedId>,
	pub _format: i64,
}

impl Entity for VersionData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 487,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VersionReturn {
	pub _format: i64,
	pub 495: Vec<Version>,
}

impl Entity for VersionReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 493,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MembershipAddData {
	#[serde(with = "serde_bytes")]
	pub 507: Vec<u8>,
	pub 2276: i64,
	pub 2277: i64,
	pub _format: i64,
	pub 508: GeneratedId,
	pub 509: GeneratedId,
}

impl Entity for MembershipAddData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 505,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ChangePasswordPostIn {
	#[serde(with = "serde_bytes")]
	pub 536: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 537: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 538: Vec<u8>,
	pub 539: Option<String>,
	#[serde(with = "serde_bytes")]
	pub 1240: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1418: Option<Vec<u8>>,
	pub 2134: i64,
	pub 2408: i64,
	pub _format: i64,
}

impl Entity for ChangePasswordPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 534,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactorAuthData {
	pub 1230: Option<i64>,
	pub 1243: Option<i64>,
	pub _format: i64,
	pub 1231: Option<U2fResponseData>,
	pub 1232: Option<IdTupleCustom>,
	pub 1905: Option<WebauthnResponseData>,
}

impl Entity for SecondFactorAuthData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 541,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactorAuthAllowedReturn {
	pub 548: bool,
	pub _format: i64,
}

impl Entity for SecondFactorAuthAllowedReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 546,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ResetPasswordPostIn {
	#[serde(with = "serde_bytes")]
	pub 586: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 587: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 588: Vec<u8>,
	pub 2135: i64,
	pub 2409: i64,
	pub _format: i64,
	pub 589: GeneratedId,
}

impl Entity for ResetPasswordPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 584,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DomainMailAddressAvailabilityData {
	pub 601: String,
	pub _format: i64,
}

impl Entity for DomainMailAddressAvailabilityData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 599,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DomainMailAddressAvailabilityReturn {
	pub 604: bool,
	pub _format: i64,
}

impl Entity for DomainMailAddressAvailabilityReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 602,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PushIdentifier {
	pub 632: i64,
	pub 633: String,
	pub 634: String,
	pub 1248: Option<DateTime>,
	pub 1476: bool,
	pub 1498: String,
	pub 1704: DateTime,
	pub 2426: i64,
	pub _area: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _owner: GeneratedId,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for PushIdentifier {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 625,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PushIdentifierList {
	pub _id: Option<CustomId>,
	pub 637: GeneratedId,
}

impl Entity for PushIdentifierList {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 635,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DeleteCustomerData {
	pub 643: bool,
	pub 644: Option<String>,
	pub 1077: Option<String>,
	#[serde(with = "serde_bytes")]
	pub 1325: Option<Vec<u8>>,
	pub _format: i64,
	pub 645: GeneratedId,
	pub 2312: Option<SurveyData>,
}

impl Entity for DeleteCustomerData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 641,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerProperties {
	pub 661: String,
	pub 975: Option<DateTime>,
	pub 2025: bool,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 922: Option<File>,
	pub 923: Option<File>,
	pub 1522: Vec<NotificationMailTemplate>,
}

impl Entity for CustomerProperties {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 656,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ExternalPropertiesReturn {
	pub 665: String,
	pub 666: i64,
	pub _format: i64,
	pub 924: Option<File>,
	pub 925: Option<File>,
}

impl Entity for ExternalPropertiesReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 663,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RegistrationCaptchaServiceData {
	pub 676: String,
	pub 677: String,
	pub _format: i64,
}

impl Entity for RegistrationCaptchaServiceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 674,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RegistrationCaptchaServiceReturn {
	pub 680: String,
	#[serde(with = "serde_bytes")]
	pub 681: Option<Vec<u8>>,
	pub _format: i64,
}

impl Entity for RegistrationCaptchaServiceReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 678,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressAlias {
	pub 686: String,
	pub 784: bool,
	pub _id: Option<CustomId>,
}

impl Entity for MailAddressAlias {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 684,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressAliasServiceData {
	pub 690: String,
	pub _format: i64,
	pub 691: GeneratedId,
}

impl Entity for MailAddressAliasServiceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 688,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressAliasServiceReturn {
	pub 694: i64,
	pub 1069: i64,
	pub 1070: i64,
	pub 1071: i64,
	pub _format: i64,
}

impl Entity for MailAddressAliasServiceReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 692,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DomainInfo {
	pub 698: String,
	pub _id: Option<CustomId>,
	pub 1044: Option<GeneratedId>,
	pub 1136: Option<GeneratedId>,
}

impl Entity for DomainInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 696,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BookingItem {
	pub 702: i64,
	pub 703: i64,
	pub 704: i64,
	pub 705: i64,
	pub 706: i64,
	pub 707: i64,
	pub 708: i64,
	pub _id: Option<CustomId>,
}

impl Entity for BookingItem {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 700,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Booking {
	pub 716: DateTime,
	pub 717: i64,
	pub 718: Option<DateTime>,
	pub 719: i64,
	pub 2103: i64,
	pub _area: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _owner: GeneratedId,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 721: Vec<BookingItem>,
}

impl Entity for Booking {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 709,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BookingsRef {
	pub _id: Option<CustomId>,
	pub 724: GeneratedId,
}

impl Entity for BookingsRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 722,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct StringWrapper {
	pub 730: String,
	pub _id: Option<CustomId>,
}

impl Entity for StringWrapper {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 728,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomDomainReturn {
	pub 733: i64,
	pub _format: i64,
	pub 734: Vec<StringWrapper>,
}

impl Entity for CustomDomainReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 731,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomDomainData {
	pub 737: String,
	pub _format: i64,
	pub 1045: Option<GeneratedId>,
}

impl Entity for CustomDomainData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 735,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InvoiceInfo {
	pub 757: Option<i64>,
	pub 758: Option<i64>,
	pub 759: bool,
	pub 1282: Option<i64>,
	pub 1283: Option<i64>,
	pub 1284: Option<i64>,
	pub 1627: Option<i64>,
	pub 1637: i64,
	pub 1638: i64,
	pub 1639: bool,
	pub 1864: Option<i64>,
	pub 2126: Option<i64>,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1640: Option<PaymentErrorInfo>,
}

impl Entity for InvoiceInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 752,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SwitchAccountTypePostIn {
	pub 774: i64,
	pub 775: Option<DateTime>,
	pub 1310: i64,
	pub 2123: Option<GeneratedId>,
	pub 2124: Option<i64>,
	pub 2496: Option<i64>,
	pub _format: i64,
	pub 2071: Option<GeneratedId>,
	pub 2314: Option<SurveyData>,
}

impl Entity for SwitchAccountTypePostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 772,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressAliasServiceDataDelete {
	pub 787: String,
	pub 788: bool,
	pub _format: i64,
	pub 789: GeneratedId,
}

impl Entity for MailAddressAliasServiceDataDelete {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 785,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PaymentDataServiceGetReturn {
	pub 792: String,
	pub _format: i64,
}

impl Entity for PaymentDataServiceGetReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 790,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PaymentDataServicePutData {
	pub 796: String,
	pub 797: String,
	pub 798: String,
	pub 799: String,
	pub 800: i64,
	pub 801: Option<String>,
	pub 802: i64,
	pub 803: Option<String>,
	pub 804: Option<String>,
	pub _format: i64,
	pub 1320: Option<CreditCard>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for PaymentDataServicePutData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 793,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PaymentDataServicePutReturn {
	pub 807: i64,
	pub _format: i64,
	pub 1840: Option<Braintree3ds2Request>,
}

impl Entity for PaymentDataServicePutReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 805,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PriceRequestData {
	pub 838: i64,
	pub 839: i64,
	pub 840: Option<bool>,
	pub 841: Option<i64>,
	pub 842: Option<i64>,
	pub 1285: bool,
	pub _id: Option<CustomId>,
}

impl Entity for PriceRequestData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 836,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PriceServiceData {
	pub 846: Option<DateTime>,
	pub _format: i64,
	pub 845: Option<PriceRequestData>,
}

impl Entity for PriceServiceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 843,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PriceItemData {
	pub 849: i64,
	pub 850: i64,
	pub 851: i64,
	pub 852: bool,
	pub _id: Option<CustomId>,
}

impl Entity for PriceItemData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 847,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PriceData {
	pub 855: i64,
	pub 856: bool,
	pub 857: i64,
	pub _id: Option<CustomId>,
	pub 858: Vec<PriceItemData>,
}

impl Entity for PriceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 853,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PriceServiceReturn {
	pub 861: DateTime,
	pub 862: Option<i64>,
	pub _format: i64,
	pub 863: Option<PriceData>,
	pub 864: Option<PriceData>,
	pub 865: Option<PriceData>,
}

impl Entity for PriceServiceReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 859,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MembershipRemoveData {
	pub _format: i64,
	pub 869: GeneratedId,
	pub 870: GeneratedId,
}

impl Entity for MembershipRemoveData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 867,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct File {
	pub 919: String,
	pub 920: String,
	#[serde(with = "serde_bytes")]
	pub 921: Vec<u8>,
	pub _id: Option<CustomId>,
}

impl Entity for File {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 917,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EmailSenderListElement {
	pub 951: String,
	pub 952: String,
	pub 953: i64,
	pub 1705: i64,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for EmailSenderListElement {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 949,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerServerProperties {
	pub 1100: bool,
	pub 1406: bool,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 959: Vec<EmailSenderListElement>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CustomerServerProperties {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 954,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateCustomerServerPropertiesData {
	#[serde(with = "serde_bytes")]
	pub 963: Vec<u8>,
	pub 2274: i64,
	pub _format: i64,
}

impl Entity for CreateCustomerServerPropertiesData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 961,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateCustomerServerPropertiesReturn {
	pub _format: i64,
	pub 966: GeneratedId,
}

impl Entity for CreateCustomerServerPropertiesReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 964,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAreaGroups {
	pub _id: Option<CustomId>,
	pub 990: GeneratedId,
}

impl Entity for UserAreaGroups {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 988,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DebitServicePutData {
	pub _format: i64,
}

impl Entity for DebitServicePutData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1041,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct EntityEventBatch {
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1085: Vec<EntityUpdate>,
}

impl Entity for EntityEventBatch {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1079,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AuditLogEntry {
	pub 1108: String,
	pub 1109: Option<String>,
	pub 1110: String,
	pub 1111: String,
	pub 1112: DateTime,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1113: Option<IdTupleGenerated>,
	pub 1307: Option<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for AuditLogEntry {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1101,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AuditLogRef {
	pub _id: Option<CustomId>,
	pub 1116: GeneratedId,
}

impl Entity for AuditLogRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1114,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WhitelabelConfig {
	pub 1133: String,
	pub 1281: String,
	pub 1308: Option<String>,
	pub 1425: Option<String>,
	pub 1496: Option<String>,
	pub 1727: String,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1252: Vec<BootstrapFeature>,
	pub 1728: Vec<StringWrapper>,
}

impl Entity for WhitelabelConfig {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1127,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BrandingDomainData {
	pub 1151: String,
	#[serde(with = "serde_bytes")]
	pub 1152: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1153: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 1154: Vec<u8>,
	pub 2161: i64,
	pub 2282: i64,
	pub _format: i64,
}

impl Entity for BrandingDomainData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1149,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BrandingDomainDeleteData {
	pub 1157: String,
	pub _format: i64,
}

impl Entity for BrandingDomainDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1155,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct U2fRegisteredDevice {
	#[serde(with = "serde_bytes")]
	pub 1164: Vec<u8>,
	pub 1165: String,
	#[serde(with = "serde_bytes")]
	pub 1166: Vec<u8>,
	pub 1167: i64,
	pub 1168: bool,
	pub _id: Option<CustomId>,
}

impl Entity for U2fRegisteredDevice {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1162,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactor {
	pub 1175: i64,
	pub 1176: String,
	#[serde(with = "serde_bytes")]
	pub 1242: Option<Vec<u8>>,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1177: Option<U2fRegisteredDevice>,
}

impl Entity for SecondFactor {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1169,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct U2fKey {
	#[serde(with = "serde_bytes")]
	pub 1180: Vec<u8>,
	pub 1181: String,
	pub _id: Option<CustomId>,
	pub 1182: IdTupleGenerated,
}

impl Entity for U2fKey {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1178,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct U2fChallenge {
	#[serde(with = "serde_bytes")]
	pub 1185: Vec<u8>,
	pub _id: Option<CustomId>,
	pub 1186: Vec<U2fKey>,
}

impl Entity for U2fChallenge {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1183,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Challenge {
	pub 1189: i64,
	pub _id: Option<CustomId>,
	pub 1190: Option<U2fChallenge>,
	pub 1247: Option<OtpChallenge>,
}

impl Entity for Challenge {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1187,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Session {
	pub 1198: String,
	pub 1199: DateTime,
	pub 1200: Option<String>,
	pub 1201: DateTime,
	#[serde(with = "serde_bytes")]
	pub 1202: Option<Vec<u8>>,
	pub 1203: i64,
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1204: Vec<Challenge>,
	pub 1205: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for Session {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1191,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAuthentication {
	pub _id: Option<CustomId>,
	pub 1208: GeneratedId,
	pub 1209: GeneratedId,
	pub 1416: Option<GeneratedId>,
}

impl Entity for UserAuthentication {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1206,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateSessionData {
	pub 1213: Option<String>,
	pub 1214: Option<String>,
	pub 1215: String,
	#[serde(with = "serde_bytes")]
	pub 1216: Option<Vec<u8>>,
	pub 1217: Option<String>,
	pub 1417: Option<String>,
	pub _format: i64,
	pub 1218: Option<GeneratedId>,
}

impl Entity for CreateSessionData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1211,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreateSessionReturn {
	pub 1221: String,
	pub _format: i64,
	pub 1222: Vec<Challenge>,
	pub 1223: GeneratedId,
}

impl Entity for CreateSessionReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1219,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct U2fResponseData {
	pub 1227: String,
	pub 1228: String,
	pub 1229: String,
	pub _id: Option<CustomId>,
}

impl Entity for U2fResponseData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1225,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactorAuthGetData {
	pub 1235: String,
	pub _format: i64,
}

impl Entity for SecondFactorAuthGetData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1233,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactorAuthGetReturn {
	pub 1238: bool,
	pub _format: i64,
}

impl Entity for SecondFactorAuthGetReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1236,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct OtpChallenge {
	pub _id: Option<CustomId>,
	pub 1246: Vec<IdTupleGenerated>,
}

impl Entity for OtpChallenge {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1244,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BootstrapFeature {
	pub 1309: i64,
	pub _id: Option<CustomId>,
}

impl Entity for BootstrapFeature {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1249,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Feature {
	pub 1255: i64,
	pub _id: Option<CustomId>,
}

impl Entity for Feature {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1253,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WhitelabelChild {
	pub 1264: String,
	pub 1265: DateTime,
	pub 1266: Option<DateTime>,
	pub 1267: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1268: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for WhitelabelChild {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1257,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WhitelabelChildrenRef {
	pub _id: Option<CustomId>,
	pub 1271: GeneratedId,
}

impl Entity for WhitelabelChildrenRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1269,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WhitelabelParent {
	pub _id: Option<CustomId>,
	pub 1274: GeneratedId,
	pub 1275: IdTupleGenerated,
}

impl Entity for WhitelabelParent {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1272,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CreditCard {
	pub 1315: String,
	pub 1316: String,
	pub 1317: String,
	pub 1318: String,
	pub 1319: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CreditCard {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1313,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct LocationServiceGetReturn {
	pub 1323: String,
	pub _format: i64,
}

impl Entity for LocationServiceGetReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1321,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct OrderProcessingAgreement {
	pub 1333: String,
	pub 1334: String,
	pub 1335: DateTime,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1336: IdTupleGenerated,
	pub 1337: GeneratedId,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for OrderProcessingAgreement {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1326,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SignOrderProcessingAgreementData {
	pub 1344: String,
	pub 1345: String,
	pub _format: i64,
}

impl Entity for SignOrderProcessingAgreementData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1342,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GeneratedIdWrapper {
	pub 1351: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for GeneratedIdWrapper {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1349,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SseConnectData {
	pub 1354: String,
	pub _format: i64,
	pub 1355: Vec<GeneratedIdWrapper>,
}

impl Entity for SseConnectData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1352,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NotificationInfo {
	pub 1366: String,
	pub 1368: GeneratedId,
	pub _id: Option<CustomId>,
	pub 2319: Option<IdTupleWrapper>,
}

impl Entity for NotificationInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1364,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RecoverCode {
	#[serde(with = "serde_bytes")]
	pub 1413: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1414: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1415: Vec<u8>,
	pub 2281: i64,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for RecoverCode {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1407,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ResetFactorsDeleteData {
	pub 1421: String,
	pub 1422: String,
	pub 1423: String,
	pub _format: i64,
}

impl Entity for ResetFactorsDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1419,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UpgradePriceServiceData {
	pub 1458: Option<DateTime>,
	pub 1459: Option<String>,
	pub _format: i64,
	pub 2077: Option<GeneratedId>,
}

impl Entity for UpgradePriceServiceData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1456,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PlanPrices {
	pub 1462: i64,
	pub 1463: i64,
	pub 1464: i64,
	pub 1465: i64,
	pub 1467: i64,
	pub 1468: i64,
	pub 2099: bool,
	pub 2100: bool,
	pub 2101: bool,
	pub 2102: i64,
	pub 2128: String,
	pub 2129: bool,
	pub _id: Option<CustomId>,
	pub 2127: PlanConfiguration,
}

impl Entity for PlanPrices {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1460,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UpgradePriceServiceReturn {
	pub 1471: Option<String>,
	pub 1472: bool,
	pub 2084: i64,
	pub _format: i64,
	pub 1473: PlanPrices,
	pub 1474: PlanPrices,
	pub 1729: PlanPrices,
	pub 1866: PlanPrices,
	pub 1867: PlanPrices,
	pub 2078: PlanPrices,
	pub 2079: PlanPrices,
	pub 2080: PlanPrices,
	pub 2081: PlanPrices,
	pub 2082: PlanPrices,
	pub 2083: PlanPrices,
	pub 2131: Vec<PlanPrices>,
}

impl Entity for UpgradePriceServiceReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1469,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RegistrationCaptchaServiceGetData {
	pub 1481: Option<String>,
	pub 1482: String,
	pub 1731: Option<String>,
	pub 1751: bool,
	pub 1752: bool,
	pub _format: i64,
}

impl Entity for RegistrationCaptchaServiceGetData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1479,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WebsocketEntityData {
	pub 1485: GeneratedId,
	pub 1486: GeneratedId,
	pub _format: i64,
	pub 1487: Vec<EntityUpdate>,
}

impl Entity for WebsocketEntityData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1483,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WebsocketCounterValue {
	pub 1490: GeneratedId,
	pub 1491: i64,
	pub _id: Option<CustomId>,
}

impl Entity for WebsocketCounterValue {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1488,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WebsocketCounterData {
	pub 1494: GeneratedId,
	pub _format: i64,
	pub 1495: Vec<WebsocketCounterValue>,
}

impl Entity for WebsocketCounterData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1492,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CertificateInfo {
	pub 1502: Option<DateTime>,
	pub 1503: i64,
	pub 1504: i64,
	pub _id: Option<CustomId>,
	pub 1505: Option<GeneratedId>,
}

impl Entity for CertificateInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1500,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NotificationMailTemplate {
	pub 1519: String,
	pub 1520: String,
	pub 1521: String,
	pub _id: Option<CustomId>,
}

impl Entity for NotificationMailTemplate {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1517,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarEventRef {
	pub 1534: CustomId,
	pub 1535: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for CalendarEventRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1532,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AlarmInfo {
	pub 1538: String,
	pub 1539: String,
	pub _id: Option<CustomId>,
	pub 1540: CalendarEventRef,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for AlarmInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1536,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAlarmInfo {
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1548: AlarmInfo,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for UserAlarmInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1541,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserAlarmInfoListType {
	pub _id: Option<CustomId>,
	pub 1551: GeneratedId,
}

impl Entity for UserAlarmInfoListType {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1549,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct NotificationSessionKey {
	#[serde(with = "serde_bytes")]
	pub 1556: Vec<u8>,
	pub _id: Option<CustomId>,
	pub 1555: IdTupleGenerated,
}

impl Entity for NotificationSessionKey {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1553,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RepeatRule {
	pub 1559: i64,
	pub 1560: i64,
	pub 1561: Option<i64>,
	pub 1562: i64,
	pub 1563: String,
	pub _id: Option<CustomId>,
	pub 2076: Vec<DateWrapper>,
	pub 2525: Vec<CalendarAdvancedRepeatRule>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for RepeatRule {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1557,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AlarmNotification {
	pub 1566: i64,
	pub 1567: String,
	pub 1568: DateTime,
	pub 1569: DateTime,
	pub _id: Option<CustomId>,
	pub 1570: AlarmInfo,
	pub 1571: Option<RepeatRule>,
	pub 1572: Vec<NotificationSessionKey>,
	pub 1573: GeneratedId,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for AlarmNotification {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1564,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AlarmServicePost {
	pub _format: i64,
	pub 1578: Vec<AlarmNotification>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for AlarmServicePost {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1576,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DnsRecord {
	pub 1583: Option<String>,
	pub 1584: i64,
	pub 1585: String,
	pub _id: Option<CustomId>,
}

impl Entity for DnsRecord {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1581,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomDomainCheckGetIn {
	pub 1588: String,
	pub _format: i64,
	pub 2053: Option<GeneratedId>,
}

impl Entity for CustomDomainCheckGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1586,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomDomainCheckGetOut {
	pub 1591: i64,
	pub _format: i64,
	pub 1592: Vec<DnsRecord>,
	pub 1593: Vec<DnsRecord>,
	pub 1758: Vec<DnsRecord>,
}

impl Entity for CustomDomainCheckGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1589,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CloseSessionServicePost {
	pub 1597: String,
	pub _format: i64,
	pub 1598: IdTupleCustom,
}

impl Entity for CloseSessionServicePost {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1595,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReceivedGroupInvitation {
	#[serde(with = "serde_bytes")]
	pub 1609: Vec<u8>,
	pub 1610: String,
	pub 1611: String,
	pub 1612: String,
	pub 1613: String,
	pub 1614: i64,
	pub 1868: Option<i64>,
	pub 2280: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1615: GeneratedId,
	pub 1616: IdTupleGenerated,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for ReceivedGroupInvitation {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1602,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserGroupRoot {
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 1624: GeneratedId,
	pub 2294: Option<KeyRotationsRef>,
	pub 2383: Option<GroupKeyUpdatesRef>,
}

impl Entity for UserGroupRoot {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1618,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PaymentErrorInfo {
	pub 1634: DateTime,
	pub 1635: String,
	pub 1636: String,
	pub _id: Option<CustomId>,
}

impl Entity for PaymentErrorInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1632,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InvoiceItem {
	pub 1643: i64,
	pub 1644: i64,
	pub 1645: Option<i64>,
	pub 1646: i64,
	pub 1647: Option<DateTime>,
	pub 1648: Option<DateTime>,
	pub 1649: bool,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for InvoiceItem {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1641,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Invoice {
	pub 1657: i64,
	pub 1658: DateTime,
	pub 1659: i64,
	pub 1660: String,
	pub 1661: String,
	pub 1662: bool,
	pub 1663: Option<String>,
	pub 1664: i64,
	pub 1665: i64,
	pub 1666: i64,
	pub 1667: i64,
	pub 1668: Option<String>,
	pub 1669: Option<String>,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1670: Vec<InvoiceItem>,
	pub 1671: GeneratedId,
	pub 1672: Vec<IdTupleGenerated>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for Invoice {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1650,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MissedNotification {
	pub 1722: Option<GeneratedId>,
	pub _format: i64,
	pub _id: Option<CustomId>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 1702: Vec<NotificationInfo>,
	pub 1703: Vec<AlarmNotification>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for MissedNotification {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1693,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BrandingDomainGetReturn {
	pub _format: i64,
	pub 1725: Option<CertificateInfo>,
}

impl Entity for BrandingDomainGetReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1723,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RejectedSender {
	pub 1742: String,
	pub 1743: String,
	pub 1744: String,
	pub 1745: String,
	pub 1746: String,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for RejectedSender {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1736,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RejectedSendersRef {
	pub _id: Option<CustomId>,
	pub 1749: GeneratedId,
}

impl Entity for RejectedSendersRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1747,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SecondFactorAuthDeleteData {
	pub _format: i64,
	pub 1757: IdTupleCustom,
}

impl Entity for SecondFactorAuthDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1755,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TakeOverDeletedAddressData {
	pub 1761: String,
	pub 1762: String,
	pub 1763: Option<String>,
	pub 1764: String,
	pub _format: i64,
}

impl Entity for TakeOverDeletedAddressData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1759,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WebsocketLeaderStatus {
	pub 1768: bool,
	pub _format: i64,
}

impl Entity for WebsocketLeaderStatus {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1766,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCard {
	pub 1776: i64,
	pub 1777: i64,
	pub 1778: String,
	pub 1779: DateTime,
	pub 1993: bool,
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

impl Entity for GiftCard {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1769,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardsRef {
	pub _id: Option<CustomId>,
	pub 1793: GeneratedId,
}

impl Entity for GiftCardsRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1791,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardOption {
	pub 1797: i64,
	pub _id: Option<CustomId>,
}

impl Entity for GiftCardOption {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1795,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardGetReturn {
	pub 1800: i64,
	pub 1801: i64,
	pub _format: i64,
	pub 1802: Vec<GiftCardOption>,
}

impl Entity for GiftCardGetReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1798,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardCreateData {
	pub 1805: String,
	#[serde(with = "serde_bytes")]
	pub 1806: Vec<u8>,
	pub 1807: i64,
	#[serde(with = "serde_bytes")]
	pub 1809: Vec<u8>,
	pub 2275: i64,
	pub _format: i64,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for GiftCardCreateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1803,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardDeleteData {
	pub _format: i64,
	pub 1812: IdTupleGenerated,
}

impl Entity for GiftCardDeleteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1810,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardCreateReturn {
	pub _format: i64,
	pub 1815: IdTupleGenerated,
}

impl Entity for GiftCardCreateReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1813,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardRedeemData {
	#[serde(with = "serde_bytes")]
	pub 1820: Vec<u8>,
	pub 1995: String,
	pub _format: i64,
	pub 1819: GeneratedId,
}

impl Entity for GiftCardRedeemData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1817,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GiftCardRedeemGetReturn {
	pub 1824: String,
	pub 1825: i64,
	pub _format: i64,
	pub 1823: IdTupleGenerated,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for GiftCardRedeemGetReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1821,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Braintree3ds2Request {
	pub 1830: String,
	pub 1831: String,
	pub 1832: String,
	pub _id: Option<CustomId>,
}

impl Entity for Braintree3ds2Request {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1828,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Braintree3ds2Response {
	pub 1835: String,
	pub 1836: String,
	pub _id: Option<CustomId>,
}

impl Entity for Braintree3ds2Response {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1833,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PaymentDataServicePostData {
	pub _format: i64,
	pub 1839: Braintree3ds2Response,
}

impl Entity for PaymentDataServicePostData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1837,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PaymentDataServiceGetData {
	pub 1863: Option<i64>,
	pub _format: i64,
}

impl Entity for PaymentDataServiceGetData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1861,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct TypeInfo {
	pub 1871: String,
	pub 1872: i64,
	pub _id: Option<CustomId>,
}

impl Entity for TypeInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1869,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ArchiveRef {
	pub 1875: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for ArchiveRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1873,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ArchiveType {
	pub _id: Option<CustomId>,
	pub 1878: TypeInfo,
	pub 1879: ArchiveRef,
	pub 1880: Vec<ArchiveRef>,
}

impl Entity for ArchiveType {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1876,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct Blob {
	pub 1884: GeneratedId,
	pub 1898: i64,
	pub 1906: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for Blob {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1882,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WebauthnResponseData {
	#[serde(with = "serde_bytes")]
	pub 1901: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1902: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1903: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 1904: Vec<u8>,
	pub _id: Option<CustomId>,
}

impl Entity for WebauthnResponseData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1899,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobReferenceTokenWrapper {
	pub 1992: String,
	pub _id: Option<CustomId>,
}

impl Entity for BlobReferenceTokenWrapper {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 1990,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerAccountTerminationRequest {
	pub 2012: DateTime,
	pub 2013: DateTime,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 2011: GeneratedId,
}

impl Entity for CustomerAccountTerminationRequest {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2005,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerAccountTerminationPostIn {
	pub 2017: Option<DateTime>,
	pub _format: i64,
	pub 2313: Option<SurveyData>,
}

impl Entity for CustomerAccountTerminationPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2015,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerAccountTerminationPostOut {
	pub _format: i64,
	pub 2020: IdTupleGenerated,
}

impl Entity for CustomerAccountTerminationPostOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2018,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressAvailability {
	pub 2028: String,
	pub 2029: bool,
	pub _id: Option<CustomId>,
}

impl Entity for MailAddressAvailability {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2026,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MultipleMailAddressAvailabilityData {
	pub _format: i64,
	pub 2032: Vec<StringWrapper>,
}

impl Entity for MultipleMailAddressAvailabilityData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2030,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MultipleMailAddressAvailabilityReturn {
	pub _format: i64,
	pub 2035: Vec<MailAddressAvailability>,
}

impl Entity for MultipleMailAddressAvailabilityReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2033,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InstanceSessionKey {
	pub 2040: GeneratedId,
	pub 2041: GeneratedId,
	#[serde(with = "serde_bytes")]
	pub 2042: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2159: Option<Vec<u8>>,
	pub 2254: i64,
	pub _id: Option<CustomId>,
	pub 2039: TypeInfo,
}

impl Entity for InstanceSessionKey {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2037,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BucketKey {
	#[serde(with = "serde_bytes")]
	pub 2045: Option<Vec<u8>>,
	#[serde(with = "serde_bytes")]
	pub 2046: Option<Vec<u8>>,
	pub 2158: i64,
	pub 2252: i64,
	pub 2253: Option<i64>,
	pub _id: Option<CustomId>,
	pub 2047: Option<GeneratedId>,
	pub 2048: Vec<InstanceSessionKey>,
}

impl Entity for BucketKey {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2043,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UpdateSessionKeysPostIn {
	pub _format: i64,
	pub 2051: Vec<InstanceSessionKey>,
}

impl Entity for UpdateSessionKeysPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2049,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReferralCodeGetIn {
	pub _format: i64,
	pub 2064: GeneratedId,
}

impl Entity for ReferralCodeGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2062,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReferralCodePostIn {
	pub _format: i64,
}

impl Entity for ReferralCodePostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2065,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReferralCodePostOut {
	pub _format: i64,
	pub 2069: GeneratedId,
}

impl Entity for ReferralCodePostOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2067,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct DateWrapper {
	pub 2075: DateTime,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for DateWrapper {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2073,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MailAddressAliasGetIn {
	pub _format: i64,
	pub 2097: GeneratedId,
}

impl Entity for MailAddressAliasGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2095,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PlanConfiguration {
	pub 2106: i64,
	pub 2107: i64,
	pub 2108: bool,
	pub 2109: bool,
	pub 2110: bool,
	pub 2111: i64,
	pub 2112: bool,
	pub 2113: bool,
	pub 2130: bool,
	pub 2136: bool,
	pub 2526: i64,
	pub _id: Option<CustomId>,
}

impl Entity for PlanConfiguration {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2104,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PlanServiceGetOut {
	pub _format: i64,
	pub 2117: PlanConfiguration,
}

impl Entity for PlanServiceGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2115,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PublicKeyPutIn {
	#[serde(with = "serde_bytes")]
	pub 2152: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2153: Vec<u8>,
	pub _format: i64,
	pub 2154: GeneratedId,
}

impl Entity for PublicKeyPutIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2150,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InvoiceDataItem {
	pub 2164: i64,
	pub 2165: i64,
	pub 2166: Option<i64>,
	pub 2167: i64,
	pub 2168: Option<DateTime>,
	pub 2169: Option<DateTime>,
	pub _id: Option<CustomId>,
}

impl Entity for InvoiceDataItem {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2162,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InvoiceDataGetOut {
	pub 2172: GeneratedId,
	pub 2173: i64,
	pub 2174: DateTime,
	pub 2175: i64,
	pub 2176: String,
	pub 2177: String,
	pub 2178: Option<String>,
	pub 2179: i64,
	pub 2180: i64,
	pub 2181: i64,
	pub 2182: i64,
	pub 2183: i64,
	pub _format: i64,
	pub 2184: Vec<InvoiceDataItem>,
}

impl Entity for InvoiceDataGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2170,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InvoiceDataGetIn {
	pub 2187: String,
	pub _format: i64,
}

impl Entity for InvoiceDataGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2185,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ChangeKdfPostIn {
	#[serde(with = "serde_bytes")]
	pub 2200: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2201: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2202: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2203: Vec<u8>,
	pub 2204: i64,
	pub 2410: i64,
	pub _format: i64,
}

impl Entity for ChangeKdfPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2198,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKey {
	#[serde(with = "serde_bytes")]
	pub 2261: Vec<u8>,
	pub 2262: i64,
	#[serde(with = "serde_bytes")]
	pub 2263: Option<Vec<u8>>,
	pub 2265: Option<i64>,
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 2266: Option<KeyPair>,
	pub 2476: Option<PubEncKeyData>,
}

impl Entity for GroupKey {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2255,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeysRef {
	pub _id: Option<CustomId>,
	pub 2269: GeneratedId,
}

impl Entity for GroupKeysRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2267,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct KeyRotation {
	pub 2289: i64,
	pub 2290: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 2482: Option<KeyMac>,
	pub 2528: Option<PubEncKeyData>,
	pub 2529: Option<KeyMac>,
	pub 2530: Option<KeyPair>,
}

impl Entity for KeyRotation {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2283,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct KeyRotationsRef {
	pub _id: Option<CustomId>,
	pub 2293: GeneratedId,
}

impl Entity for KeyRotationsRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2291,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct SurveyData {
	pub 2297: i64,
	pub 2298: i64,
	pub 2299: Option<String>,
	pub 2300: i64,
	pub _id: Option<CustomId>,
}

impl Entity for SurveyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2295,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct IdTupleWrapper {
	pub 2317: GeneratedId,
	pub 2318: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for IdTupleWrapper {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2315,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserGroupKeyDistribution {
	#[serde(with = "serde_bytes")]
	pub 2326: Vec<u8>,
	pub 2327: i64,
	pub _format: i64,
	pub _id: Option<GeneratedId>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
}

impl Entity for UserGroupKeyDistribution {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2320,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeyRotationData {
	pub 2332: i64,
	#[serde(with = "serde_bytes")]
	pub 2333: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2334: Option<Vec<u8>>,
	pub 2335: Option<i64>,
	pub _id: Option<CustomId>,
	pub 2336: GeneratedId,
	pub 2337: Option<KeyPair>,
	pub 2397: Vec<GroupKeyUpdateData>,
	pub 2432: Vec<GroupMembershipUpdateData>,
}

impl Entity for GroupKeyRotationData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2328,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeyRotationPostIn {
	pub _format: i64,
	pub 2340: Vec<GroupKeyRotationData>,
}

impl Entity for GroupKeyRotationPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2338,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeyRotationInfoGetOut {
	pub 2344: bool,
	pub _format: i64,
	pub 2407: Vec<IdTupleGenerated>,
}

impl Entity for GroupKeyRotationInfoGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2342,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct RecoverCodeData {
	pub 2348: i64,
	#[serde(with = "serde_bytes")]
	pub 2349: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2350: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2351: Vec<u8>,
	pub _id: Option<CustomId>,
}

impl Entity for RecoverCodeData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2346,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserGroupKeyRotationData {
	#[serde(with = "serde_bytes")]
	pub 2354: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2355: Vec<u8>,
	pub 2356: i64,
	#[serde(with = "serde_bytes")]
	pub 2357: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2359: Option<Vec<u8>>,
	pub 2360: i64,
	#[serde(with = "serde_bytes")]
	pub 2362: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2550: Option<Vec<u8>>,
	pub _id: Option<CustomId>,
	pub 2358: KeyPair,
	pub 2361: GeneratedId,
	pub 2363: Option<RecoverCodeData>,
	pub 2470: Option<PubEncKeyData>,
}

impl Entity for UserGroupKeyRotationData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2352,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AdminGroupKeyRotationPostIn {
	pub _format: i64,
	pub 2366: GroupKeyRotationData,
	pub 2367: UserGroupKeyRotationData,
	pub 2483: Vec<KeyMac>,
	pub 2535: Vec<AdminGroupKeyDistributionElement>,
}

impl Entity for AdminGroupKeyRotationPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2364,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeyUpdate {
	#[serde(with = "serde_bytes")]
	pub 2377: Vec<u8>,
	pub 2378: i64,
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	#[serde(with = "serde_bytes")]
	pub _ownerEncSessionKey: Option<Vec<u8>>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _ownerKeyVersion: Option<i64>,
	pub _permissions: GeneratedId,
	pub 2379: BucketKey,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for GroupKeyUpdate {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2369,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeyUpdatesRef {
	pub _id: Option<CustomId>,
	pub 2382: GeneratedId,
}

impl Entity for GroupKeyUpdatesRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2380,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PubEncKeyData {
	pub 2386: String,
	#[serde(with = "serde_bytes")]
	pub 2387: Vec<u8>,
	pub 2388: i64,
	pub 2389: Option<i64>,
	pub 2390: i64,
	pub 2469: i64,
	pub 2551: Option<String>,
	pub 2552: Option<i64>,
	pub _id: Option<CustomId>,
	pub 2553: Option<KeyMac>,
}

impl Entity for PubEncKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2384,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupKeyUpdateData {
	pub 2393: i64,
	#[serde(with = "serde_bytes")]
	pub 2394: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2395: Vec<u8>,
	pub _id: Option<CustomId>,
	pub 2396: PubEncKeyData,
}

impl Entity for GroupKeyUpdateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2391,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupMembershipKeyData {
	pub 2401: i64,
	pub 2402: i64,
	#[serde(with = "serde_bytes")]
	pub 2403: Vec<u8>,
	pub _id: Option<CustomId>,
	pub 2400: GeneratedId,
}

impl Entity for GroupMembershipKeyData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2398,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct MembershipPutIn {
	pub _format: i64,
	pub 2406: Vec<GroupMembershipKeyData>,
}

impl Entity for MembershipPutIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2404,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct GroupMembershipUpdateData {
	#[serde(with = "serde_bytes")]
	pub 2430: Vec<u8>,
	pub 2431: i64,
	pub _id: Option<CustomId>,
	pub 2429: GeneratedId,
}

impl Entity for GroupMembershipUpdateData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2427,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AffiliatePartnerKpiMonthSummary {
	pub 2455: i64,
	pub 2456: i64,
	pub 2457: i64,
	pub 2458: i64,
	pub 2459: i64,
	pub 2460: i64,
	pub _id: Option<CustomId>,
}

impl Entity for AffiliatePartnerKpiMonthSummary {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2453,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AffiliatePartnerKpiServiceGetOut {
	pub 2463: String,
	pub 2464: i64,
	pub 2465: i64,
	pub _format: i64,
	pub 2466: Vec<AffiliatePartnerKpiMonthSummary>,
}

impl Entity for AffiliatePartnerKpiServiceGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2461,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UserGroupKeyRotationPostIn {
	pub _format: i64,
	pub 2473: UserGroupKeyRotationData,
}

impl Entity for UserGroupKeyRotationPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2471,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct KeyMac {
	pub 2480: i64,
	#[serde(with = "serde_bytes")]
	pub 2481: Vec<u8>,
	pub 2527: i64,
	pub _id: Option<CustomId>,
	pub 2479: GeneratedId,
}

impl Entity for KeyMac {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2477,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AppStoreSubscriptionGetOut {
	pub 2499: i64,
	pub _format: i64,
}

impl Entity for AppStoreSubscriptionGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2497,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AppStoreSubscriptionGetIn {
	pub 2502: String,
	pub _format: i64,
}

impl Entity for AppStoreSubscriptionGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2500,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VerifierTokenServiceOut {
	pub 2512: String,
	pub _format: i64,
}

impl Entity for VerifierTokenServiceOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2510,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct VerifierTokenServiceIn {
	#[serde(with = "serde_bytes")]
	pub 2519: Vec<u8>,
	pub _format: i64,
}

impl Entity for VerifierTokenServiceIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2517,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CalendarAdvancedRepeatRule {
	pub 2523: i64,
	pub 2524: String,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CalendarAdvancedRepeatRule {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2521,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AdminGroupKeyDistributionElement {
	pub _id: Option<CustomId>,
	pub 2533: GeneratedId,
	pub 2534: PubEncKeyData,
}

impl Entity for AdminGroupKeyDistributionElement {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2531,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AdminGroupKeyRotationPutIn {
	pub _format: i64,
	pub 2538: KeyMac,
	pub 2539: KeyPair,
}

impl Entity for AdminGroupKeyRotationPutIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2536,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PubDistributionKey {
	#[serde(with = "serde_bytes")]
	pub 2544: Vec<u8>,
	#[serde(with = "serde_bytes")]
	pub 2545: Vec<u8>,
	pub _id: Option<CustomId>,
	pub 2542: GeneratedId,
	pub 2543: KeyMac,
}

impl Entity for PubDistributionKey {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2540,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct AdminGroupKeyRotationGetOut {
	pub _format: i64,
	pub 2548: Vec<GeneratedId>,
	pub 2549: Vec<PubDistributionKey>,
}

impl Entity for AdminGroupKeyRotationGetOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "sys",
			type_id: 2546,
		}
	}
}
