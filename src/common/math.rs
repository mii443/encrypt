use std::ops::Div;

use bigdecimal::{BigDecimal, FromPrimitive, Zero, One};

fn extGCD(a: BigDecimal, b: BigDecimal) -> (BigDecimal, BigDecimal, BigDecimal) {
    let mut a = a;
    let mut b = b;
    let mut x0 = BigDecimal::from_i32(1).unwrap();
    let mut y0 = BigDecimal::from_i32(0).unwrap();
    let mut x1 = BigDecimal::from_i32(0).unwrap();
    let mut y1 = BigDecimal::from_i32(1).unwrap();

    while b != BigDecimal::zero() {
        let q = down(a.clone() / b.clone());
        let at = a.clone();
        a = plusMod(at, b.clone());

        let x0t = x0.clone();
        x0 = x1.clone();
        x1 = x0t - q.clone() * x1.clone();

        let y0t = y0.clone();
        y0 = y1.clone();
        y1 = y0t - q.clone() * y1;
    }
    return (a, x0, y0);
}

pub fn plusMod(a: BigDecimal, b: BigDecimal) -> BigDecimal {
    a.clone() - floor(a / b.clone()) * b
}

pub fn modInv(a: BigDecimal, m: BigDecimal) -> BigDecimal {
    let r = extGCD(a, m);
    if r.0 != BigDecimal::one() {
        panic!("Moduler inverse does not exist.");
    } else {

        return plusMod(r.1, m);
    }
}

pub fn down(a: BigDecimal) -> BigDecimal {
    return a.clone() - a % BigDecimal::one();
}

pub fn floor(a: BigDecimal) -> BigDecimal {

    let m = a.clone() % BigDecimal::one();

    if a > BigDecimal::zero() {
        return a.clone() - m;
    }

    if m < BigDecimal::zero() {
        return a - m - BigDecimal::one();
    }

    return a;
}