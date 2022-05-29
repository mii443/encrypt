use bigdecimal::{num_bigint::BigInt, Zero, One};

pub fn plus_mod(a: BigInt, m: BigInt) -> BigInt {
    (a.clone() % m.clone() + m.clone()) % m
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
