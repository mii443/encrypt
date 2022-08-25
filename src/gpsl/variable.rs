use crate::{
    common::finite_field::FiniteFieldElement,
    elliptic_curve::encryption::EncryptedEllipticCurvePoint,
};
use primitive_types::U512;
use serde::{Deserialize, Serialize};

use super::gpsl_type::GPSLType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Variable {
    Number {
        value: i64,
    },
    Text {
        value: String,
    },
    Return {
        value: Box<Variable>,
    },
    PureEncrypted {
        value: EncryptedEllipticCurvePoint,
    },
    PairedEncrypted {
        a: FiniteFieldElement,
        b: FiniteFieldElement,
        c: FiniteFieldElement,
        d: FiniteFieldElement,
    },
    U512 {
        value: U512,
    },
    Vec {
        value: Vec<Variable>,
        gpsl_type: GPSLType,
    },
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
            Variable::Vec { gpsl_type, .. } => gpsl_type.to_str(),
            Variable::None { .. } => "none".to_string(),
        }
    }

    pub fn extract_number(&self) -> Option<i64> {
        match self {
            Variable::Number { value } => Some(*value),
            _ => None,
        }
    }

    pub fn extract_text(&self) -> Option<String> {
        match self {
            Variable::Text { value } => Some(value.clone()),
            Variable::Number { value } => Some(value.to_string()),
            Variable::PureEncrypted { value } => Some(value.to_string()),
            Variable::PairedEncrypted { a, b, c, d } => Some(format!(
                "{:x}{:x}{:x}{:x}",
                a.value, b.value, c.value, d.value
            )),
            Variable::U512 { value } => Some(value.to_string()),
            Variable::Vec { value, .. } => {
                let mut result = String::new();
                for v in value {
                    result.push_str(&v.extract_text().unwrap());
                }
                Some(result)
            }
            _ => None,
        }
    }
}
