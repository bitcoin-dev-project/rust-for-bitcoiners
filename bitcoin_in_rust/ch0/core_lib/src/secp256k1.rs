use num_bigint::BigUint;
use std::ops::{Add, Mul, Sub};
use lazy_static::lazy_static;

// Field element struct
#[derive(Clone, Debug, PartialEq)]
struct FieldElement {
    value: BigUint,
}


lazy_static! {
    static ref P: BigUint = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
}

impl FieldElement {
    fn new(value: BigUint) -> Self {
        FieldElement {
            value: value % &*P,
        }
    }
}

// Implement Add trait for field element addition
impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        FieldElement::new((self.value + other.value) % &*P)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        FieldElement::new((self.value * other.value) % &*P)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        FieldElement::new((self.value - other.value) % &*P)
    }
}

// Elliptic curve point struct
#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: FieldElement,
    y: FieldElement,
}

impl Point {
    fn new(x: FieldElement, y: FieldElement) -> Self {
        Point { x, y }
    }

    fn is_zero(&self) -> bool {
        self.x.value == BigUint::ZERO && self.y.value == BigUint::ZERO
    }

    fn double(&self) -> Self {
        if self.is_zero() {
            return self.clone();
        }

        let a = FieldElement::new(BigUint::ZERO);

        let x = &self.x.value;
        let y = &self.y.value;

        let lambda = FieldElement::new((x * x * BigUint::from(3u32) + a.value) * (y * BigUint::from(2u32)).modpow(&(&*P - BigUint::from(2u32)), &*P));
        let x3 = lambda.clone() * lambda.clone() - self.x.clone() - self.x.clone();
        let y3 = lambda * (self.x.clone() - x3.clone()) - self.y.clone();

        Point::new(x3, y3)
    }

    fn add(&self, other: &Self) -> Self {
        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        let x1 = &self.x.value;
        let y1 = &self.y.value;
        let x2 = &other.x.value;
        let y2 = &other.y.value;

        if x1 == x2 {
            if y1 == y2 {
                return self.double();
            } else {
                return Point::new(FieldElement::new(BigUint::ZERO), FieldElement::new(BigUint::ZERO));
            }
        }

        let lambda = FieldElement::new((y2 - y1) * (x2 - x1).modpow(&(&*P - BigUint::from(2u32)), &*P));
        let x3 = lambda.clone() * lambda.clone() - self.x.clone() - other.x.clone();
        let y3 = lambda * (self.x.clone() - x3.clone()) - self.y.clone();

        Point::new(x3, y3)
    }
}
