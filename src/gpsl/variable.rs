use crate::{
    common::finite_field::FiniteFieldElement,
    elliptic_curve::encryption::EncryptedEllipticCurvePoint,
};
use primitive_types::U512;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Variable {
    Number { value: i64 },
    Text { value: String },
    Return { value: Box<Variable> },
    PureEncrypted { value: EncryptedEllipticCurvePoint },
    PairedEncrypted { value: FiniteFieldElement },
    U512 { value: U512 },
    None {},
}

impl Variable {
    pub fn get_type(&self) -> String {
        match self {
            Variable::Number { .. } => "num".to_string(),
            Variable::Text { .. } => "String".to_string(),
            Variable::Return { .. } => "return".to_string(),
            Variable::PureEncrypted { .. } => "eep".to_string(),
            Variable::PairedEncrypted { .. } => "eep_p".to_string(),
            Variable::U512 { .. } => "U512".to_string(),
            Variable::None { .. } => "none".to_string(),
        }
    }
}
