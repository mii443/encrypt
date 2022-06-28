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

impl EllipticCurvePoint {

    pub fn exp(&self, k: U512) -> Self {
        if k == U512::zero() {
            return Self::Infinity;
        }

        let mut g = self.clone();
        let s = k.bits();
        let mut i = 1usize;
        while i < s {
            println!("{}, {}", i, k.bit(i));
            g = g + g;
            if k.bit(i) {
                g = g + (*self);
            }
            println!("{}", g);
            i += 1;
        }
        g
    }

    pub fn is_inf(&self) -> bool {
        match self {
            EllipticCurvePoint::Point { .. } => false,
            EllipticCurvePoint::Infinity => true,
        }
    }

    pub fn extract(&self) -> (FiniteFieldElement, FiniteFieldElement, FiniteFieldElement, FiniteFieldElement) {
        match self {
            EllipticCurvePoint::Point { x, y, a, b } => (*x,*y,*a,*b),
            _ => panic!("inifinity")
        }
    }

    pub fn lambda(p: EllipticCurvePoint, q: EllipticCurvePoint) -> FiniteFieldElement {
        let (x1, y1) = match p {
            EllipticCurvePoint::Point { x, y, .. } => (x,y),
            _ => panic!("P is inifinity.")
        };

        let (x2, y2) = match q {
            EllipticCurvePoint::Point { x, y, .. } => (x,y),
            _ => panic!("Q is inifinity.")
        };

        (y2 - y1) / (x2 - x1)
    }

    pub fn l(g: EllipticCurvePoint, h: EllipticCurvePoint, var: EllipticCurvePoint) -> FiniteFieldElement {
        //println!("L g: {}, h: {}, var: {}", g, h, var);
        let (gx, gy, a, _) = g.extract();
        let (hx, hy, _, _) = h.extract();
        let (varx, vary, _, _) = var.extract();

        let v = if g == h {
            (FiniteFieldElement::new(U512::from(3u8), gx.p) * gx * gx + a) * (FiniteFieldElement::new(U512::from(2u8), gx.p) * gy).inverse()
        } else {
            (hy - gy) * (hx - gx).inverse()
        };
        //println!("result: {}", vary - (v * (varx - gx) + gy));
        vary - (v * (varx - gx) + gy)
    }

    pub fn v(r: EllipticCurvePoint, v: EllipticCurvePoint) -> FiniteFieldElement {
        let (rx, _, _, _) = r.extract();
        let (vx, _, _, _) = v.extract();

        //println!("V g: {}, var: {}, result: {}", r, v, vx - rx);
        vx - rx
    }

    pub fn g(p: EllipticCurvePoint, q: EllipticCurvePoint, v: EllipticCurvePoint) -> FiniteFieldElement {
        let (px, py, _, _) = p.extract();
        let (qx, qy, _, _) = q.extract();
        let (vx, vy, _, _) = v.extract();
        if px == qx && py == -qy {
            vx - px
        } else if p == q {
            let vinv = Self::v(p + p, v).inverse();
            Self::l(p, p, v) * vinv
        } else {
            let vinv = Self::v(p + q, v).inverse();
            Self::l(p, q, v) * vinv
        }
    }

    pub fn miller(p: EllipticCurvePoint, q: EllipticCurvePoint, m: U512) -> FiniteFieldElement {
        //println!("Miller Start: {}, {}, {}", p, q, m);
        let (px, _, _, _) = p.extract();
        let prime = px.p;

        let mut f = FiniteFieldElement::new(U512::from(1u8), prime);
        let mut t = p.clone();

        let mut s = m.bits();
        let mut i = 1usize;
        //println!("1 to {}", s);
        while i < s {
            //println!("ARR: {}", m.bit(i));
            //println!("I: {}", i);
            let gf = Self::g(t,t,q);
            f = f * f * gf;
            //println!("Miller g: {}", gf);
            t = t + t;
            if m.bit(i) {
                f = f * Self::g(t, p,q);
                t = t + p;
            }
            i += 1;
        }

        f
    }

    pub fn weil(p: EllipticCurvePoint, q: EllipticCurvePoint, m: U512) -> FiniteFieldElement {
        let (px, _, _, _) = p.extract();
        if p == q {
            FiniteFieldElement::new(U512::one(), px.p)
        } else if p.is_inf() || q.is_inf() {
            FiniteFieldElement::new(U512::one(), px.p)
        } else {
            let minv = Self::miller(q, p, m).inverse();
            -Self::miller(p, q, m) * minv
        }
    }
}

impl Display for EllipticCurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let EllipticCurvePoint::Point { x, y, .. } = self {
            write!(f, "({}, {})", x.value, y.value)
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
                            let t = x1 * x1 * FiniteFieldElement::new(U512::from(3u8), p) + a;
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
