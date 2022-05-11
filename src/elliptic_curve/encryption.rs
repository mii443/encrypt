use bigdecimal::{BigDecimal, FromPrimitive, Num, Zero};
use crate::common::finite_field::FiniteFieldElement;

use crate::{ffe, b};

use super::elliptic_curve::{EllipticCurve, EllipticCurvePoint};

pub struct Encryption {
    pub ellictic_curve: EllipticCurve,
    pub base_point: EllipticCurvePoint,
    pub order: BigDecimal
}

impl Encryption {
    pub fn plain_to_ec_point(&self, m: BigDecimal) -> EllipticCurvePoint {
        if m == BigDecimal::zero() {
            return EllipticCurvePoint {
                x: ffe!(0, 1),
                y: ffe!(0, 1),
                infinity: true
            };
        }

        return self.ellictic_curve.clone().times(self.base_point.clone(), m);
    }
}
