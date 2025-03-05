// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReadCounterData {
	pub 14: String,
	pub 15: Option<GeneratedId>,
	pub 299: i64,
	pub _format: i64,
}

impl Entity for ReadCounterData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 12,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReadCounterReturn {
	pub 18: Option<i64>,
	pub _format: i64,
	pub 304: Vec<CounterValue>,
}

impl Entity for ReadCounterReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 16,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct WriteCounterData {
	pub 51: String,
	pub 52: GeneratedId,
	pub 53: i64,
	pub 215: Option<i64>,
	pub _format: i64,
}

impl Entity for WriteCounterData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 49,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ApprovalMail {
	pub 227: Option<String>,
	pub 228: Option<DateTime>,
	pub 229: String,
	pub _format: i64,
	pub _id: Option<IdTupleCustom>,
	pub _ownerGroup: Option<GeneratedId>,
	pub _permissions: GeneratedId,
	pub 230: Option<GeneratedId>,
}

impl Entity for ApprovalMail {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 221,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct CounterValue {
	pub 302: GeneratedId,
	pub 303: i64,
	pub _id: Option<CustomId>,
}

impl Entity for CounterValue {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 300,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ErrorReportFile {
	pub 307: String,
	pub 308: String,
	pub _id: Option<CustomId>,
}

impl Entity for ErrorReportFile {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 305,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ErrorReportData {
	pub 318: DateTime,
	pub 319: String,
	pub 320: i64,
	pub 321: Option<String>,
	pub 322: String,
	pub 323: Option<String>,
	pub 324: String,
	pub 325: Option<String>,
	pub 326: String,
	pub _id: Option<CustomId>,
}

impl Entity for ErrorReportData {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 316,
		}
	}
}

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct ReportErrorIn {
	pub _format: i64,
	pub 337: ErrorReportData,
	pub 338: Vec<ErrorReportFile>,
}

impl Entity for ReportErrorIn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "monitor",
			type_id: 335,
		}
	}
}
