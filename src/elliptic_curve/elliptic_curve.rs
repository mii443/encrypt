use bigdecimal::{BigDecimal, FromPrimitive};
use crate::{common::finite_field::*, b, ffeb, ffe};

#[derive(Debug, Clone, PartialEq)]
pub struct EllipticCurve {
    pub a: BigDecimal,
    pub b: BigDecimal,
    pub p: BigDecimal
}

#[derive(Debug, Clone, PartialEq)]
pub struct EllipticCurvePoint {
    pub x: FiniteFieldElement,
    pub y: FiniteFieldElement,
    pub infinity: bool
}

impl EllipticCurve {
    pub fn add(self, lhs: EllipticCurvePoint, rhs: EllipticCurvePoint) -> EllipticCurvePoint {
        let (x1, y1) = (rhs.x.clone(), rhs.y.clone());
        let (x2, y2) = (lhs.x.clone(), lhs.y.clone());
        let p = x1.p.clone();

        if x1.clone() == x2.clone() && y2.clone() == -y1.clone() {
            return EllipticCurvePoint {
                x: ffe!(0, 1),
                y: ffe!(0, 1),
                infinity: true
            }
        }

        if lhs.infinity {
            return rhs
        }

        if rhs.infinity {
            return lhs
        }

        let l = if x1.clone() == x2.clone() && y1.clone() == y2.clone() {
            (
                x1.clone().pow(ffeb!(b!(2), p.clone()))
                * ffeb!(b!(3), p.clone())
                + ffeb!(self.a, p.clone())
            ).floor_div(
                y1.clone() * ffeb!(b!(2), p.clone())
            )
        } else {
            (y2 - y1.clone()).floor_div(x2.clone() - x1.clone())
        };

        let x3 = l.clone().pow(ffeb!(b!(2), p.clone())) - x1.clone() - x2.clone();
        let y3 = l * (x1 - x3.clone()) - y1;

        EllipticCurvePoint {
            x: x3,
            y: y3,
            infinity: false
        }
    }
}
