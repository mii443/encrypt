use std::ops::{Mul, Add, Sub, Div};

use super::elliptic_curve::{EllipticCurve, EllipticCurvePoint};

#[derive(Debug)]
pub struct Encryption<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialEq
{
    pub ellictic_curve: EllipticCurve<T>,
    pub base_point: EllipticCurvePoint<T>,
    pub order: T
}

impl<T> Encryption<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialEq
{
    pub fn plain_to_ec_point<U>(&self, m: U) -> EllipticCurvePoint<T>
    where
        U: Sub<Output = U> + Div<Output = U> + Copy + PartialEq + PartialOrd
    {
        if m == m - m {
            return EllipticCurvePoint::Infinity
        }

        return self.base_point * m;
    }
}
