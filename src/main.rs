use encrypt::{elliptic_curve::{elliptic_curve::{EllipticCurve, EllipticCurvePoint}, encryption::Encryption}, common::finite_field::FiniteFieldElement};
use primitive_types::U512;

fn main() {
    println!("Encryption Library");

    let p = U512::from_str_radix("115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(0u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from(7u8), p);
    let secp256_k1_base_x = FiniteFieldElement::new(U512::from_str_radix("55066263022277343669578718895168534326250603453777594175500187360389116729240", 10).unwrap(), p);
    let secp256_k1_base_y = FiniteFieldElement::new(U512::from_str_radix("32670510020758816978083085130507043184471273380659243275938904335757337482424", 10).unwrap(), p);
    let secp256_k1_order = FiniteFieldElement::new(U512::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494337", 10).unwrap(), p);

/*
    let p = U512::from_str_radix("5", 10).unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(1u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from(1u8), p);
    let secp256_k1_base_x = FiniteFieldElement::new(U512::from_str_radix("0", 10).unwrap(), p);
    let secp256_k1_base_y = FiniteFieldElement::new(U512::from_str_radix("1", 10).unwrap(), p);
    let secp256_k1_order = FiniteFieldElement::new(U512::from_str_radix("2", 10).unwrap(), p);
*/
    let ec = EllipticCurve {
        a: secp256_k1_a,
        b: secp256_k1_b
    };

    let encryption = Encryption {
        ellictic_curve: ec,
        base_point: ec.point(
            secp256_k1_base_x,
            secp256_k1_base_y,
        ),
        order: secp256_k1_order,
        plain_mapping: vec![]
    };

    let twenty = encryption.plain_to_ec_point(U512::from(12u8));
    let ten = encryption.plain_to_ec_point(U512::from(10u8));
    let two = encryption.plain_to_ec_point(U512::from(2u8));
    println!("{:?}", twenty);
    println!("{:?}", ten + two);
    println!("{:?}", encryption.ec_point_to_plain(ten));
/*
    let t = encryption.base_point + encryption.base_point;
    println!("{:?}", t);
    println!("{:?}", encryption.base_point);
    */
}
