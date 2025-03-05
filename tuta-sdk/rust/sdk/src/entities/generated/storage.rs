// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobGetIn {
	pub 52: GeneratedId,
	pub 110: Option<GeneratedId>,
	pub _format: i64,
	pub 193: Vec<BlobId>,
}

impl Entity for BlobGetIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 50,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobWriteData {
	pub 75: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for BlobWriteData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 73,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobAccessTokenPostIn {
	pub 180: Option<i64>,
	pub _format: i64,
	pub 80: Option<BlobWriteData>,
	pub 181: Option<BlobReadData>,
}

impl Entity for BlobAccessTokenPostIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 77,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobAccessTokenPostOut {
	pub _format: i64,
	pub 161: BlobServerAccessInfo,
}

impl Entity for BlobAccessTokenPostOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 81,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobReferencePutIn {
	pub 97: Option<GeneratedId>,
	pub 107: GeneratedId,
	pub 123: i64,
	pub _format: i64,
	pub 122: Vec<super::sys::BlobReferenceTokenWrapper>,
}

impl Entity for BlobReferencePutIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 94,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobReferenceDeleteIn {
	pub 102: Option<GeneratedId>,
	pub 103: GeneratedId,
	pub 124: i64,
	pub _format: i64,
	pub 105: Vec<super::sys::Blob>,
}

impl Entity for BlobReferenceDeleteIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 100,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobPostOut {
	pub 127: Option<String>,
	pub _format: i64,
	pub 208: Vec<super::sys::BlobReferenceTokenWrapper>,
}

impl Entity for BlobPostOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 125,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobArchiveRef {
	pub _format: i64,
	pub _id: Option<IdTupleGenerated>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 135: GeneratedId,
}

impl Entity for BlobArchiveRef {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 129,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobId {
	pub 146: GeneratedId,
	pub _id: Option<CustomId>,
}

impl Entity for BlobId {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 144,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobServerUrl {
	pub 156: String,
	pub _id: Option<CustomId>,
}

impl Entity for BlobServerUrl {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 154,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobServerAccessInfo {
	pub 159: String,
	pub 192: DateTime,
	pub 209: i64,
	pub _id: Option<CustomId>,
	pub 160: Vec<BlobServerUrl>,
}

impl Entity for BlobServerAccessInfo {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 157,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct InstanceId {
	pub 174: Option<GeneratedId>,
	pub _id: Option<CustomId>,
}

impl Entity for InstanceId {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 172,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct BlobReadData {
	pub 177: GeneratedId,
	pub 178: Option<GeneratedId>,
	pub _id: Option<CustomId>,
	pub 179: Vec<InstanceId>,
}

impl Entity for BlobReadData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "storage",
			type_id: 175,
		}
	}
}
