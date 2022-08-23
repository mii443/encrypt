/*
[6139062701328441600,
[258929920560, 23709360],
[[Mod(3308825380872319861, 6139062703770505681), Mod(4839630718792142583, 6139062703770505681)],
[Mod(4767914906170010398, 6139062703770505681), Mod(2445476831433994309, 6139062703770505681)]]]
 */

fn o_main() {
    let p = U512::from_str_radix("1009", 10).unwrap();

    let secp256_k1_a = FiniteFieldElement::new(U512::from(37u8), p);
    let secp256_k1_b = FiniteFieldElement::new(U512::from_str_radix("0", 10).unwrap(), p);

    let pp = {
        let x = FiniteFieldElement::new(U512::from_str_radix("417", 10).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("952", 10).unwrap(), p);
        EllipticCurvePoint::Point {
            x,
            y,
            a: secp256_k1_a,
            b: secp256_k1_b,
        }
    };
    let pd = {
        let x = FiniteFieldElement::new(U512::from_str_radix("561", 10).unwrap(), p);
        let y = FiniteFieldElement::new(U512::from_str_radix("153", 10).unwrap(), p);
        EllipticCurvePoint::Point {
            x,
            y,
            a: secp256_k1_a,
            b: secp256_k1_b,
        }
    };

    let r = U512::from_str_radix("7", 10).unwrap();

    let f = EllipticCurvePoint::weil(pp, pd, r);

    let s = U512::from(10u8);
    let sd = U512::from(5u8);

    let q = pp * s;
    let qd = pd * sd;

    let ra = U512::from_str_radix("20", 10).unwrap();
    let rad = U512::from_str_radix("26", 10).unwrap();

    let m = U512::from_str_radix("2", 10).unwrap();
    let md = U512::from_str_radix("2", 10).unwrap();

    let s1 = pp * m + q * ra;
    let t1 = pp * ra;
    let s2 = pd * md + qd * rad;
    let t2 = pd * rad;

    let a = EllipticCurvePoint::weil(s1, s2, r);
    let b = EllipticCurvePoint::weil(s1, t2, r);
    let c = EllipticCurvePoint::weil(t1, s2, r);
    let d = EllipticCurvePoint::weil(t1, t2, r);

    let dec = a * d.pow(s * sd) / b.pow(sd) / c.pow(s) * f;

    println!("{} * {} = {}", m, md, search(f, dec));
}

pub fn search(base: FiniteFieldElement, target: FiniteFieldElement) -> U512 {
    let mut i = U512::one();
    let mut b = base;
    println!("{}, {}", base, target);
    while b != target {
        b = b * base;
        i += U512::one();
    }
    if i < U512::from(7u8) {
        i
    } else {
        U512::zero()
    }
}
