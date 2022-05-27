use std::ops::{Mul, Add, Sub, Div};

use primitive_types::U512;

use super::elliptic_curve::{EllipticCurve, EllipticCurvePoint};

#[derive(Debug)]
pub struct Encryption<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialEq
{
    pub ellictic_curve: EllipticCurve<T>,
    pub base_point: EllipticCurvePoint<T>,
    pub order: T,
    pub plain_mapping: Vec<EllipticCurvePoint<T>>
}

impl<T> Encryption<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialEq
{
    pub fn ec_point_to_plain(&self, point: EllipticCurvePoint<T>) -> U512 {
        println!("ec point to plain");
        match point {
            EllipticCurvePoint::Infinity => {
                return U512::from(0u8)
            }
            _ => {}
        }

        let mut x = 0i64;

        println!("get plain mapping");
        for p in &self.plain_mapping {
            match p {
                EllipticCurvePoint::Point { x: px, y: py, a, b } => {
                    match point {
                        EllipticCurvePoint::Point { x: ppx, y, a, b } => {
                            if *px == ppx && *py == y {
                                return U512::from(x) + U512::from(1u8);
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            x += 1;
        }

        x -= 1;

        let mut tmp = if x < 0 {
            EllipticCurvePoint::Infinity {}
        } else {
            self.plain_mapping[x as usize]
        };

        println!("calc mapping");
        while x < i64::MAX && !(match tmp {
            EllipticCurvePoint::Point { x: tx, y: ty, a, b } => match point {
                EllipticCurvePoint::Point { x: px, y: py, a, b } => tx == px && ty == py,
                _ => false
            },
            EllipticCurvePoint::Infinity => match point {
                EllipticCurvePoint::Infinity => true,
                _ => false
            },
        }) {
            x += 1;
            tmp = tmp + self.base_point;
        }

        U512::from(x + 1)
    }

    pub fn plain_to_ec_point(&self, m: U512) -> EllipticCurvePoint<T> {
        if m == U512::from(0) {
            return EllipticCurvePoint::Infinity
        }

        return self.base_point * m;
    }
}
