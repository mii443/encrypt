use bigdecimal::{num_bigint::BigInt, Zero, One};
use primitive_types::U512;

use super::finite_field::FiniteFieldElement;
use bigdecimal::Num;

pub fn plus_mod(a: BigInt, m: BigInt) -> BigInt {
    (a.clone() % m.clone() + m.clone()) % m
}

pub fn u512_to_bigint(i: U512) -> BigInt {
    BigInt::from_str_radix(&format!("{}", i), 10).unwrap()
}

pub fn bigint_to_u512(i: BigInt) -> U512 {
    U512::from_str_radix(&format!("{}", i), 10).unwrap()
}

pub fn abs(a: BigInt, p: BigInt) -> BigInt {
    if a >= BigInt::zero() {
        a % p
    } else {
        (p.clone()-(-a)%p.clone())%p
    }
}

pub fn pow_mod(a: BigInt, n: BigInt, p: BigInt) -> BigInt {
    let mut r = BigInt::one();
    let mut db = a.clone();
    let mut n = n;

    while n > BigInt::zero() {
        if (n.clone() & BigInt::one()) == BigInt::one() {
            r = (r.clone() * db.clone()) % p.clone();
        }
        db = (db.clone() * db.clone()) % p.clone();
        n = n >> 1;
    }

    abs(r, p)
}

pub fn random_n_q(p: BigInt) -> BigInt {
    let mut i = BigInt::one();
    let k = (p.clone() - BigInt::one()) >> 1i32;
    while i < p {
        println!("pm {:?}", FiniteFieldElement::new(bigint_to_u512(i.clone()), bigint_to_u512(p.clone())).pow(bigint_to_u512(k.clone())).value);
        if bigint_to_u512(pow_mod(i.clone(),k.clone(),p.clone())) != U512::one() {
            break;
        }
        i += BigInt::one();
    }
    i
}

pub fn mod_inv(a: BigInt, m: BigInt) -> BigInt {
    let mut a = a;
    if a < BigInt::zero() {
        a = a.modpow(&BigInt::one(), &m);
    }

    let mut x = BigInt::zero();
    let mut y = BigInt::one();
    let mut gcd = m.clone();
    let mut px = BigInt::one();
    let mut py = BigInt::zero();
    let mut pgcd = a;

    while gcd.clone() > BigInt::zero() {
        let q = pgcd.clone() / gcd.clone();

        let tmp = x.clone();
        x = px.clone() - q.clone() * tmp.clone();
        px = tmp.clone();

        let tmp = y.clone();
        y = py.clone() - q.clone() * tmp.clone();
        py = tmp.clone();

        let tmp = gcd.clone();
        gcd = pgcd.clone() - q.clone() * tmp.clone();
        pgcd = tmp.clone();
    }

    plus_mod(px, m)
}
