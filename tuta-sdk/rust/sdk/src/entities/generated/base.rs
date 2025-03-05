// @generated
#![allow(non_snake_case, unused_imports)]
use super::super::*;
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(PartialEq, Debug))]
pub struct PersistenceResourcePostReturn {
	pub 2: Option<GeneratedId>,
	pub 3: GeneratedId,
	pub _format: i64,
}

impl Entity for PersistenceResourcePostReturn {
	fn type_ref() -> TypeRef {
		TypeRef {
			app: "base",
			type_id: 0,
		}
	}
}
