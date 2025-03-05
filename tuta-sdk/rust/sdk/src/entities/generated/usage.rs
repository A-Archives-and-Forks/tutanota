// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestMetricConfigValue {
	pub 10: String,
	pub 11: String,
	pub _id: Option<CustomId>,
}

impl Entity for UsageTestMetricConfigValue {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 8,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestMetricConfig {
	pub 14: String,
	pub 15: i64,
	pub _id: Option<CustomId>,
	pub 16: Vec<UsageTestMetricConfigValue>,
}

impl Entity for UsageTestMetricConfig {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 12,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestMetricData {
	pub 19: String,
	pub 20: String,
	pub _id: Option<CustomId>,
}

impl Entity for UsageTestMetricData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 17,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestStage {
	pub 37: String,
	pub 87: i64,
	pub 88: i64,
	pub _id: Option<CustomId>,
	pub 38: Vec<UsageTestMetricConfig>,
}

impl Entity for UsageTestStage {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 35,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestAssignmentIn {
	pub 55: Option<GeneratedId>,
	pub _format: i64,
}

impl Entity for UsageTestAssignmentIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 53,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestAssignment {
	pub 58: GeneratedId,
	pub 59: String,
	pub 60: Option<i64>,
	pub 61: bool,
	pub _id: Option<CustomId>,
	pub 62: Vec<UsageTestStage>,
}

impl Entity for UsageTestAssignment {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 56,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestAssignmentOut {
	pub 65: GeneratedId,
	pub _format: i64,
	pub 66: Vec<UsageTestAssignment>,
}

impl Entity for UsageTestAssignmentOut {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 63,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct UsageTestParticipationIn {
	pub 82: GeneratedId,
	pub 83: i64,
	pub 84: GeneratedId,
	pub _format: i64,
	pub 85: Vec<UsageTestMetricData>,
}

impl Entity for UsageTestParticipationIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "usage",
			type_id: 80,
		}
	}
}
