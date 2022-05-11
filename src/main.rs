use encrypt::{common::finite_field::FiniteFieldElement, elliptic_curve::{elliptic_curve::{EllipticCurve, EllipticCurvePoint}, encryption::Encryption}, b, ffe, ffeb};
use bigdecimal::{BigDecimal, FromPrimitive, Num};
// 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
fn main() {
    println!("Encryption Library");

    let secp256_k1_a: BigDecimal = BigDecimal::from_str_radix("0", 10).unwrap();
    let secp256_k1_b: BigDecimal = BigDecimal::from_str_radix("7", 10).unwrap();
    let secp256_k1_p: BigDecimal = BigDecimal::from_str_radix("115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();
    let secp256_k1_base_x: BigDecimal = BigDecimal::from_str_radix("55066263022277343669578718895168534326250603453777594175500187360389116729240", 10).unwrap();
    let secp256_k1_base_y: BigDecimal = BigDecimal::from_str_radix("32670510020758816978083085130507043184471273380659243275938904335757337482424", 10).unwrap();
    let secp256_k1_order: BigDecimal = BigDecimal::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494337", 10).unwrap();

    let ec = EllipticCurve {
        a: secp256_k1_a,
        b: secp256_k1_b,
        p: secp256_k1_p.clone(),
    };

    let encryption = Encryption {
        ellictic_curve: ec,
        base_point: EllipticCurvePoint {
            x: ffeb!(secp256_k1_base_x, secp256_k1_p.clone()),
            y: ffeb!(secp256_k1_base_y, secp256_k1_p),
            infinity: false
         },
         order: secp256_k1_order
    };

    println!("{:?}", encryption.plain_to_ec_point(BigDecimal::from_i32(2).unwrap()));

    /*
    for x in 1..101 {
        for y in 1..101 {
            println!("{}, {}: {}", x, y, encrypt::common::math::plusMod(BigDecimal::from_i32(x).unwrap(), BigDecimal::from_i32(y).unwrap()));
        }
    }

    println!("{}", encrypt::common::math::plusMod(BigDecimal::from_i32(100).unwrap(), BigDecimal::from_i32(50).unwrap()));
*/
}
