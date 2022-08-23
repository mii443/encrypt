use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
    sync::mpsc,
    thread,
};

use primitive_types::{U256, U512};
use serde::{Deserialize, Serialize};

use crate::common::finite_field::FiniteFieldElement;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use super::elliptic_curve::{EllipticCurve, EllipticCurvePoint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encryption {
    pub ellictic_curve: EllipticCurve,
    pub base_point: EllipticCurvePoint,
    pub order: FiniteFieldElement,
    pub plain_mapping: Vec<EllipticCurvePoint>,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct EncryptedEllipticCurvePoint {
    pub data: EllipticCurvePoint,
    pub rp: EllipticCurvePoint,
}

impl EncryptedEllipticCurvePoint {
    pub fn default() -> Self {
        Self {
            data: EllipticCurvePoint::Infinity,
            rp: EllipticCurvePoint::Infinity,
        }
    }
}

impl Display for EncryptedEllipticCurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data {
            EllipticCurvePoint::Infinity => write!(f, "Infinity"),
            EllipticCurvePoint::Point { x, y, .. } => write!(f, "{:x}", x.value + y.value),
        }
    }
}

impl Add for EncryptedEllipticCurvePoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data + rhs.data,
            rp: self.rp + rhs.rp,
        }
    }
}

impl Sub for EncryptedEllipticCurvePoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data + (-rhs.data),
            rp: self.rp + (-rhs.rp),
        }
    }
}

impl Mul<U512> for EncryptedEllipticCurvePoint {
    type Output = Self;

    fn mul(self, rhs: U512) -> Self::Output {
        let mut tmp = self;
        let mut point: Option<EncryptedEllipticCurvePoint> = None;
        let mut n = rhs;
        while n > U512::zero() {
            if n & U512::one() == U512::one() {
                if let Some(s_point) = point {
                    point = Some(s_point + tmp);
                } else {
                    point = Some(tmp);
                }
            }

            n = n >> 1;
            tmp = tmp + tmp;
        }
        point.unwrap()
    }
}

impl Encryption {
    pub fn secp256k1() -> Self {
        let p = U512::from_str_radix(
            "115792089237316195423570985008687907853269984665640564039457584007908834671663",
            10,
        )
        .unwrap();

        let secp256_k1_a = FiniteFieldElement::new(U512::from(0u8), p);
        let secp256_k1_b = FiniteFieldElement::new(U512::from(7u8), p);
        let secp256_k1_base_x = FiniteFieldElement::new(
            U512::from_str_radix(
                "55066263022277343669578718895168534326250603453777594175500187360389116729240",
                10,
            )
            .unwrap(),
            p,
        );
        let secp256_k1_base_y = FiniteFieldElement::new(
            U512::from_str_radix(
                "32670510020758816978083085130507043184471273380659243275938904335757337482424",
                10,
            )
            .unwrap(),
            p,
        );
        let secp256_k1_order = FiniteFieldElement::new(
            U512::from_str_radix(
                "115792089237316195423570985008687907852837564279074904382605163141518161494337",
                10,
            )
            .unwrap(),
            p,
        );
        let ec = EllipticCurve {
            a: secp256_k1_a,
            b: secp256_k1_b,
        };

        Self {
            ellictic_curve: ec,
            base_point: ec.point(secp256_k1_base_x, secp256_k1_base_y),
            order: secp256_k1_order,
            plain_mapping: vec![],
        }
    }

    pub fn ec_point_to_plain(&self, point: EllipticCurvePoint) -> U512 {
        match point {
            EllipticCurvePoint::Infinity => return U512::from(0u8),
            _ => {}
        }

        let mut x = 0i64;

        for p in &self.plain_mapping {
            match p {
                EllipticCurvePoint::Point {
                    x: px,
                    y: py,
                    a: _,
                    b: _,
                } => match point {
                    EllipticCurvePoint::Point {
                        x: ppx,
                        y,
                        a: _,
                        b: _,
                    } => {
                        if *px == ppx && *py == y {
                            return U512::from(x) + U512::from(1u8);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            x += 1;
        }

        x -= 1;

        let mut tmp = if x < 0 {
            EllipticCurvePoint::Infinity {}
        } else {
            self.plain_mapping[x as usize]
        };

        while x < i64::MAX
            && !(match tmp {
                EllipticCurvePoint::Point {
                    x: tx,
                    y: ty,
                    a: _,
                    b: _,
                } => match point {
                    EllipticCurvePoint::Point {
                        x: px,
                        y: py,
                        a: _,
                        b: _,
                    } => tx == px && ty == py,
                    _ => false,
                },
                EllipticCurvePoint::Infinity => match point {
                    EllipticCurvePoint::Infinity => true,
                    _ => false,
                },
            })
        {
            x += 1;
            tmp = tmp + self.base_point;
        }

        U512::from(x + 1)
    }

    pub fn plain_to_ec_point(&self, m: U512) -> EllipticCurvePoint {
        if m == U512::from(0u8) {
            return EllipticCurvePoint::Infinity;
        }

        return self.base_point * m;
    }

    pub fn decrypt(ecc_p: EncryptedEllipticCurvePoint, private_key: U512) -> EllipticCurvePoint {
        let rq = ecc_p.rp * private_key;
        ecc_p.data + (-rq)
    }

    pub fn encrypt(
        &self,
        message: EllipticCurvePoint,
        public_key: EllipticCurvePoint,
        r: Option<U512>,
    ) -> EncryptedEllipticCurvePoint {
        let ra = if let Some(ra) = r { ra } else { Self::random() };

        let (data_tx, data_rx) = mpsc::channel();
        let (rp_tx, rp_rx) = mpsc::channel();

        let s = self.clone();
        thread::spawn(move || {
            let val = s.base_point * ra;
            rp_tx.send(val).unwrap();
        });

        thread::spawn(move || {
            let val = message + public_key * ra;
            data_tx.send(val).unwrap();
        });

        let data_received = data_rx.recv().unwrap();
        let rp_received = rp_rx.recv().unwrap();

        EncryptedEllipticCurvePoint {
            data: data_received,
            rp: rp_received,
        }
    }

    pub fn get_public_key(&self, private_key: U512) -> EllipticCurvePoint {
        self.base_point * private_key
    }

    pub fn get_private_key() -> U512 {
        Self::random()
    }

    pub fn random() -> U512 {
        let mut csprng = ChaCha20Rng::from_entropy();

        let mut data = [0u8; 32];
        csprng.fill_bytes(&mut data);

        U512::from(U256::from(data))
    }
}
