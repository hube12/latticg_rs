use num_bigint::BigInt;
use std::ops::{Neg, Div};
use num_traits::{Signed, Zero, One, ToPrimitive};
use num_integer::Integer;
use core::hash;
use std::cmp::Ordering;

#[derive( Debug)]
pub struct BigFraction {
    ntor: BigInt,
    dtor: BigInt,
}

// Note: derived `Clone` doesn't specialize `clone_from`,
// but we want to keep the allocation in `data`.
impl Clone for BigFraction {
    #[inline]
    fn clone(&self) -> Self {
        BigFraction {
            ntor: self.ntor.clone(),
            dtor: self.dtor.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.ntor.clone_from(&other.ntor);
        self.dtor.clone_from(&other.dtor);
    }
}

impl hash::Hash for BigFraction {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.ntor.hash(state);
        self.dtor.hash(state);
    }
}


impl PartialEq for BigFraction {
    #[inline]
    fn eq(&self, other: &BigFraction) -> bool {
        self.get_denominator().eq(&other.get_denominator()) && self.get_numerator().eq(&other.get_numerator())
    }
}

impl Eq for BigFraction {}

impl PartialOrd for BigFraction {
    #[inline]
    fn partial_cmp(&self, other: &BigFraction) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigFraction {
    #[inline]
    fn cmp(&self, other: &BigFraction) -> Ordering {
        (self.get_numerator()*other.get_denominator()).cmp(&(self.get_denominator() * other.get_numerator()))
    }
}


impl<'a> BigFraction {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Result<Self, &'a str> {
        if denominator.signum() == BigInt::zero() {
            Err("Dividing by zero")
        } else {
            Ok(Self::simplify(BigFraction { ntor: numerator.clone(), dtor: denominator.clone() }))
        }
    }

    pub fn new_integer(numerator:BigInt) -> Result<Self, &'a str> {
        Self::new(numerator,One::one())
    }

    pub fn parse(s:String) -> Option<BigInt> {
        // TODO FIXME
        BigInt::parse_bytes(&*s.into_bytes(),10)
    }

    pub fn get_denominator(&self) -> BigInt {
        self.dtor.clone()
    }

    pub fn get_numerator(&self) -> BigInt {
        self.ntor.clone()
    }

    pub fn to_double(&self)->f64{
        self.ntor.to_f64().expect("no numerator") / self.dtor.to_f64().expect("no denominator")
    }

    pub fn simplify(mut fraction: BigFraction) -> BigFraction {
        if fraction.ntor.signum() == BigInt::zero() {
            fraction.dtor = BigInt::one();
            return fraction;
        }
        if fraction.dtor.signum() == -BigInt::one() {
            fraction.ntor = (&fraction.ntor).neg();

            fraction.dtor = (&fraction.dtor).neg();
        }
        let common_factor: BigInt = fraction.ntor.gcd(&fraction.dtor);
        fraction.ntor = (&fraction.ntor).div(&common_factor);
        fraction.dtor = (&fraction.dtor).div(&common_factor);
        fraction
    }
}