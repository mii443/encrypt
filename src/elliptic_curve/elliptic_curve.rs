use std::ops::{Div, Add, Sub, Mul};

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

impl Add for EllipticCurvePoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.clone() {
            EllipticCurvePoint::Point { x: x1, y: y1, a, b } => {
                match rhs {
                    EllipticCurvePoint::Point { x: x2, y: y2, a: a2, b: b2 } => {
                        println!("default plus");
                        
                        let p = x1.p;
                        if a != a2 || b != b2 {
                            panic!("Cannot add different curve point.");
                        }

                        if x1 == x2 && y2 == y1 - y1 - y1 {
                            return EllipticCurvePoint::Infinity
                        }

                        let l = if x1 == x2 && y1 == y2 {
                            println!("twice");
                            let t = x1 * x1 * FiniteFieldElement::new(U512::from(3), p) + a;
                            let u = y1 * FiniteFieldElement::new(U512::from(2), p);
                            let a = t / u;
                            println!("t: {:?}\nu: {:?}\na: {:?}", t, u, a);
                            a
                        } else {
                            println!("plus");
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
        let mut n = rhs;
        let mut r: EllipticCurvePoint = EllipticCurvePoint::Infinity;
        while n > U512::from(0) {
            r = r + self;
            n = n - U512::from(1);
        }

        r
    }
}
