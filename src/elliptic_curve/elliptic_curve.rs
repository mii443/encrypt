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
        if lhs.clone().x == rhs.x && rhs.y == -lhs.clone().y {
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

        let (psi, phi) = if lhs == rhs {
            let phi = Self::twice_phi(lhs.clone(), self.clone());
            let psi = Self::twice_psi(lhs.clone(), self);

            (phi, psi)
        } else {
            let phi = Self::add_phi(lhs.clone(), rhs.clone());
            let psi = Self::add_psi(lhs.clone(), rhs.clone());

            (phi, psi)
        };

        EllipticCurvePoint {
            x: phi.clone() * phi - lhs.x.clone() - rhs.x.clone(),
            y: todo!(),
            infinity: todo!(), 
        }
    }

    fn add_phi(a: EllipticCurvePoint, b: EllipticCurvePoint) -> FiniteFieldElement {
        (a.y - b.y) / (a.x - b.x)
    }

    fn add_psi(a: EllipticCurvePoint, b: EllipticCurvePoint) -> FiniteFieldElement {
        (a.x.clone() * b.y - b.x.clone() * a.y) / (a.x - b.x)
    }

    fn twice_phi(a: EllipticCurvePoint, c: EllipticCurve) -> FiniteFieldElement {
        let x = a.x.clone();
        let y = a.y.clone();
        let p = a.x.p.clone();
        (
            ffeb!(b!(3), p.clone()) * x.clone() * x
             + ffeb!(c.a, p.clone())
        ) / (
            ffeb!(b!(2), p) * y
        )
    }

    fn twice_psi(a: EllipticCurvePoint, c: EllipticCurve) -> FiniteFieldElement {
        let p = a.x.p.clone();
        (
            ffeb!(b!(3), p.clone()) * a.x.clone() * a.x.clone()
             + ffeb!(c.a, p.clone()) * a.x.clone()
             - ffeb!(b!(2), p.clone()) * a.y.clone() * a.y.clone()
        ) / (
            ffeb!(b!(2), p) * a.y
        )
    }
}
