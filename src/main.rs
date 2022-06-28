
use encrypt::{elliptic_curve::elliptic_curve::EllipticCurvePoint, common::finite_field::FiniteFieldElement};
use primitive_types::U512;

fn main() {
    let p = U512::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFEE37", 16).unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(0u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from(3u8), p);

    let g = {
        let x = FiniteFieldElement::new(U512::from_str_radix("DB4FF10EC057E9AE26B07D0280B7F4341DA5D1B1EAE06C7D", 16).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("9B2F2F6D9C5628A7844163D015BE86344082AA88D95E2F9D", 16).unwrap(), p);
        EllipticCurvePoint::Point { x, y, a: secp256_k1_a, b: secp256_k1_b }
    };
    let p = g * U512::from_str_radix("2343432432243", 10).unwrap();
    let q = g * U512::from_str_radix("4233434343432443243", 10).unwrap();
    let r = U512::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFE26F2FC170F69466A74DEFD8D", 16).unwrap();

    let f = EllipticCurvePoint::weil(p, q, r);
    let f1 = EllipticCurvePoint::weil(p.exp(U512::from(2u8)), q.exp(U512::from(1u8)), r);

    println!("{}", search(f, f1));
}

pub fn search(base: FiniteFieldElement, target: FiniteFieldElement) -> U512 {
    let mut i = U512::one();
    let mut b = base;
    println!("{}, {}", base, target);
    while b != target {
        b = b * base;
        i += U512::one();
    }
    i
}