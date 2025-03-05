// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerAccountPosting {
	pub 81: i64,
	pub 82: DateTime,
	pub 83: Option<String>,
	pub 84: i64,
	pub _id: Option<CustomId>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CustomerAccountPosting {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "accounting",
			type_id: 79,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CustomerAccountReturn {
	pub 92: i64,
	pub 94: i64,
	pub _format: i64,
	pub _ownerGroup: Option<GeneratedId>,
	#[serde(with = "serde_bytes")]
	pub _ownerPublicEncSessionKey: Option<Vec<u8>>,
	pub _publicCryptoProtocolVersion: Option<i64>,
	pub 90: Vec<CustomerAccountPosting>,
	pub _errors: Option<Errors>,
	pub _finalIvs: HashMap<String, FinalIv>,
}

impl Entity for CustomerAccountReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "accounting",
			type_id: 86,
		}
	}
}
