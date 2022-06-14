use bigdecimal::num_bigint::BigInt;
use encrypt::{elliptic_curve::{elliptic_curve::EllipticCurve, encryption::Encryption}, common::{finite_field::FiniteFieldElement, math::{random_n_q, mod_sqrt}}};
use primitive_types::U512;

fn main() {
    println!("Encryption Library");

    println!("{}", random_n_q(BigInt::from(23)));
    println!("{}", mod_sqrt(BigInt::from(4), BigInt::from(23)));

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
    let secp256_k1_order = FiniteFieldElement::nimage.pngew(U512::from_str_radix("2", 10).unwrap(), p);
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

    let private_key = Encryption::get_private_key();
    println!("private_key: {:x}", private_key);
    let public_key = encryption.get_public_key(private_key);
    println!("public_key: {}", public_key);

    for x in 0..10 {
        let ten = encryption.plain_to_ec_point(U512::from(10u32));
        let e_ten = encryption.encrypt(ten, public_key, None);
        println!("10 -> {}", e_ten.data);
    }

    let two = encryption.plain_to_ec_point(U512::from(2u32));
    let e_two = encryption.encrypt(two, public_key, None);
    println!("2 -> {}", e_two.data);
/* 
    println!("10 + 2 -> {}", (e_ten + e_two).data);
    println!("decrypt: {:?}", encryption.ec_point_to_plain(Encryption::decrypt(e_ten + e_two, private_key)));
*/
    /*
    let twenty = encryption.plain_to_ec_point(U512::from(12u8));
    let ten = encryption.plain_to_ec_point(U512::from(10u8));
    let two = encryption.plain_to_ec_point(U512::from(2u8));
    println!("{:?}", twenty);
    println!("{:?}", encryption.ec_point_to_plain(twenty));
    println!("{:?}", ten + two);
    println!("{:?}", encryption.ec_point_to_plain(ten + two));
    */
}
