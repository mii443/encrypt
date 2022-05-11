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

    let ecp2 = EllipticCurvePoint {
        x: ffe!(4, 5),
        y: ffe!(2, 5),
        infinity: false
    };

    for x in 0..1000 {
        println!("{:?}", ec.clone().add(ecp1.clone(), ecp2.clone()));
    }
    /*
    for x in 1..101 {
        for y in 1..101 {
            println!("{}, {}: {}", x, y, encrypt::common::math::plusMod(BigDecimal::from_i32(x).unwrap(), BigDecimal::from_i32(y).unwrap()));
        }
    }

    println!("{}", encrypt::common::math::plusMod(BigDecimal::from_i32(100).unwrap(), BigDecimal::from_i32(50).unwrap()));
*/
}
