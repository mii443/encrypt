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

