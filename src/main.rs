use encrypt::{common::finite_field::FiniteFieldElement, elliptic_curve::elliptic_curve::{EllipticCurve, EllipticCurvePoint}, b, ffe};
use bigdecimal::{BigDecimal, FromPrimitive};

fn main() {
    println!("Encryption Library");

    let ec = EllipticCurve {
        a: b!(1),
        b: b!(1),
        p: b!(5),
    };

    let ecp1 = EllipticCurvePoint {
        x: ffe!(0, 5),
        y: ffe!(1, 5),
        infinity: false
    };

    println!("{:?}", ec.add(ecp1.clone(), ecp1));
}
