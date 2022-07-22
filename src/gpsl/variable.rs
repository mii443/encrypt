use crate::{
    common::finite_field::FiniteFieldElement,
    elliptic_curve::encryption::EncryptedEllipticCurvePoint,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Variable {
    Number { value: i64 },
    Text { value: String },
    Return { value: Box<Variable> },
    PureEncrypted { value: EncryptedEllipticCurvePoint },
    PairedEncrypted { value: FiniteFieldElement },
    None {},
}
