use std::{ops::{Add, Mul, Neg}, fmt::Display};

use primitive_types::U512;

use crate::common::finite_field::FiniteFieldElement;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticCurve {
    pub a: FiniteFieldElement,
    pub b: FiniteFieldElement
}

impl EllipticCurve {
    pub fn point(self, x: FiniteFieldElement, y: FiniteFieldElement) -> EllipticCurvePoint {
        EllipticCurvePoint::Point { x, y, a: self.a, b: self.b }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EllipticCurvePoint {
    Point {
        x: FiniteFieldElement,
        y: FiniteFieldElement,
        a: FiniteFieldElement,
        b: FiniteFieldElement
    },
    Infinity
}

impl Display for EllipticCurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let EllipticCurvePoint::Point { x, y, a, b } = self {
            write!(f, "({:x}, {:x})", x.value, y.value)
        } else {
            write!(f, "Infinity")
        }
    }
}

impl EllipticCurvePoint {
    pub fn check(self) -> bool {
        match self {
            EllipticCurvePoint::Point { x, y, a, b } => {
                y * y == x * x * x + a * x + b
            },
            EllipticCurvePoint::Infinity => true,
        }
    }
}

impl Neg for EllipticCurvePoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let EllipticCurvePoint::Point { x, y, a, b } = self {
            EllipticCurvePoint::Point { x, y: -y, a, b }
        } else {
            return self
        }
    }
}

impl Add for EllipticCurvePoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.clone() {
            EllipticCurvePoint::Point { x: x1, y: y1, a, b } => {
                match rhs {
                    EllipticCurvePoint::Point { x: x2, y: y2, a: a2, b: b2 } => {
                        let p = x1.p;
                        if a != a2 || b != b2 {
                            panic!("Cannot add different curve point.");
                        }

                        if x1 == x2 && y2 == y1 - y1 - y1 {
                            return EllipticCurvePoint::Infinity
                        }

                        let l = if x1 == x2 && y1 == y2 {
                            let t = x1 * x1 * FiniteFieldElement::new(U512::from(3), p) + a;
                            let u = y1 * FiniteFieldElement::new(U512::from(2), p);
                            let a = t / u;
                            a
                        } else {
                            (y2 - y1) / (x2 - x1)
                        };
                        let x = l * l - x1 - x2;
                        let y = l * (x1 - x) - y1;

                        EllipticCurvePoint::Point { x, y, a, b }
                    },
                    EllipticCurvePoint::Infinity => self
                }
            },
            EllipticCurvePoint::Infinity => rhs
        }
    }
}

impl Mul<U512> for EllipticCurvePoint {
    type Output = Self;

    fn mul(self, rhs: U512) -> Self::Output {
        let mut tmp = self;
        let mut point = EllipticCurvePoint::Infinity;
        let mut n = rhs;
        while n > U512::zero() {
            if n & U512::one() == U512::one() {
                point = point + tmp;
            }

            n = n >> 1;
            tmp = tmp + tmp;
        }
        point
    }
}
