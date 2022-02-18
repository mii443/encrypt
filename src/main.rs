use bigdecimal::{BigDecimal, FromPrimitive};
use encrypt::finite_field::FiniteFieldElement;

macro_rules! b {
    ( $x: expr ) => {
        BigDecimal::from_i64($x).unwrap()
    };
}

macro_rules! ffe {
    ( $value: expr, $p: expr ) => {
        FiniteFieldElement::new(b!($value), b!($p))
    };
}

fn main() {
    let a = ffe!(4, 5);
    let b = ffe!(3, 5);
    println!("{}", a + b);
}
