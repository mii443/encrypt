use bigdecimal::BigDecimal;
use crate::common::finite_field::*;

pub struct EllipticCurve {
    pub a: BigDecimal,
    pub b: BigDecimal,
    pub p: BigDecimal
}

pub struct EllipticCurvePoint {
    pub x: FiniteFieldElement,
    pub y: FiniteFieldElement
}

impl EllipticCurve {
    pub fn add(lhs: EllipticCurvePoint, rhs: EllipticCurvePoint) -> EllipticCurvePoint {
        lhs
    }

    fn add_phi(a: EllipticCurvePoint, b: EllipticCurvePoint) -> FiniteFieldElement {
        
    }
}
