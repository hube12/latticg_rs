#![allow(dead_code, unused_imports)]

use num_bigint::{BigInt};
use core::{fmt, hash};
use num_traits::{Signed, One, Zero, ToPrimitive};
use std::ops::{Mul, Add, Sub, Neg, Div};
use num_integer::Integer;
use std::error::Error;
use std::cmp::Ordering;

// This is a junky implementation, the real implementation is getting worked on (2k+ lines)

#[derive(Debug)]
pub struct BigFraction {
    ntor: BigInt,
    dtor: BigInt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBigFractionError {
    kind: BigFractionErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BigFractionErrorKind {
    Empty,
    InvalidDigit,
    DivideByZero,
}

impl ParseBigFractionError {
    fn __description(&self) -> &str {
        use self::BigFractionErrorKind::*;
        match self.kind {
            Empty => "cannot parse integer from empty string",
            InvalidDigit => "invalid digit found in string",
            DivideByZero => "Dividing by zero"
        }
    }

    fn empty() -> Self {
        ParseBigFractionError {
            kind: BigFractionErrorKind::Empty,
        }
    }

    fn invalid() -> Self {
        ParseBigFractionError {
            kind: BigFractionErrorKind::InvalidDigit,
        }
    }
    fn divide_by_zero() -> Self {
        ParseBigFractionError {
            kind: BigFractionErrorKind::DivideByZero,
        }
    }
}

impl fmt::Display for ParseBigFractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

#[cfg(feature = "std")]
impl Error for ParseBigFractionError {
    fn description(&self) -> &str {
        self.__description()
    }
}

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

impl<'a> BigFraction {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Result<Self, ParseBigFractionError> {
        if denominator.signum() == BigInt::zero() {
            Err(ParseBigFractionError::divide_by_zero())
        } else {
            Ok(Self::simplify(BigFraction { ntor: numerator.clone(), dtor: denominator.clone() }))
        }
    }
    pub fn get_zero() -> BigFraction {
        BigFraction::new(BigInt::from(0), BigInt::from(1)).expect("This should not fail")
    }
    pub fn get_one() -> BigFraction {
        BigFraction::new(BigInt::from(1), BigInt::from(1)).expect("This should not fail")
    }
    pub fn get_minus_one() -> BigFraction {
        BigFraction::new(BigInt::from(-1), BigInt::from(1)).expect("This should not fail")
    }
    pub fn get_half() -> BigFraction {
        BigFraction::new(BigInt::from(1), BigInt::from(2)).expect("This should not fail")
    }
    pub fn get_pi() -> BigFraction {
        BigFraction::new(BigInt::from(30246273033735921u64), BigInt::from(9627687726852338u64)).expect("This should not fail")
    }
    pub fn get_log_pi() -> BigFraction {
        BigFraction::new(BigInt::from(14405300475444212u64), BigInt::from(12584017114880639u64)).expect("This should not fail")
    }
    pub fn get_log_10() -> BigFraction {
        BigFraction::new(BigInt::from(152469287047331902u64), BigInt::from(66216570024379193u64)).expect("This should not fail")
    }
    pub fn get_exp() -> BigFraction {
        BigFraction::new(BigInt::from(47813267563899719u64), BigInt::from(17589518151988078u64)).expect("This should not fail")
    }
    pub fn get_two() -> BigFraction {
        BigFraction::new(BigInt::from(2), BigInt::from(1)).expect("This should not fail")
    }

    pub fn new_integer(numerator: BigInt) -> Result<Self, ParseBigFractionError> {
        Self::new(numerator, One::one())
    }

    pub fn parse(s: String) -> Option<BigInt> {
        BigInt::parse_bytes(&*s.into_bytes(), 10)
    }

    pub fn add(self, other: BigFraction) -> Self {
        let hi = self.ntor.mul(other.dtor.clone()).add(other.ntor.mul(&self.dtor));
        let lo = self.dtor.mul(&other.dtor);
        return BigFraction::new(hi, lo).expect("An error occurred while adding");
    }

    pub fn add_int(self, other: BigInt) -> Self {
        let hi = self.ntor.add(other.mul(&self.dtor));
        let lo = self.dtor.clone();
        return BigFraction::new(hi, lo).expect("An error occurred while adding");
    }


    pub fn sub(self, other: BigFraction) -> Self {
        let hi = self.ntor.mul(other.dtor.clone()).sub(other.ntor.mul(&self.dtor));
        let lo = self.dtor.mul(&other.dtor);
        return BigFraction::new(hi, lo).expect("An error occurred while subtracting");
    }

    pub fn sub_int(self, other: BigInt) -> Self {
        let hi = self.ntor.sub(other.mul(&self.dtor));
        let lo = self.dtor.clone();
        return BigFraction::new(hi, lo).expect("An error occurred while subtracting");
    }

    pub fn mul(self, other: BigFraction) -> Self {
        let hi = self.ntor.mul(&other.ntor);
        let lo = self.dtor.mul(&other.dtor);
        return BigFraction::new(hi, lo).expect("An error occurred while multiplying");
    }

    pub fn mul_int(self, other: BigInt) -> Self {
        let hi = self.ntor.mul(other);
        let lo = self.dtor.clone();
        return BigFraction::new(hi, lo).expect("An error occurred while multiplying");
    }

    pub fn div(self, other: BigFraction) -> Self {
        let hi = self.ntor.mul(&other.dtor);
        let lo = self.dtor.mul(&other.ntor);
        return BigFraction::new(hi, lo).expect("An error occurred while dividing");
    }

    pub fn div_int(self, other: BigInt) -> Self {
        let hi = self.ntor.clone();
        let lo = self.dtor.mul(other);
        return BigFraction::new(hi, lo).expect("An error occurred while dividing");
    }

    pub fn negate(self) -> Self {
        return BigFraction::new(self.ntor.neg(), self.dtor).expect("An error occurred while negating");
    }

    pub fn reciprocal(self) -> Self {
        return BigFraction::new(self.dtor, self.ntor).expect("An error occurred while getting the reciprocal");
    }

    pub fn floor(self) -> BigInt {
        if self.dtor.eq(&BigInt::one()) {
            self.ntor
        } else if self.ntor.signum() == -BigInt::one() {
            self.ntor.div(self.dtor).sub(BigInt::one())
        } else {
            self.ntor.div(self.dtor)
        }
    }

    pub fn ceil(self) -> BigInt {
        if self.dtor.eq(&BigInt::one()) {
            self.ntor
        } else if self.ntor.signum() == BigInt::one() {
            self.ntor.div(self.dtor).sub(BigInt::one())
        } else {
            self.ntor.div(self.dtor)
        }
    }

    pub fn round(self) -> BigInt {
        self.add(Self::get_half()).floor()
    }

    pub fn signum(self) -> BigInt {
        self.ntor.signum()
    }

    pub fn abs(self) -> BigFraction {
        if self.ntor.signum() == -BigInt::one() {
            self.negate()
        } else {
            self
        }
    }

    pub fn exp(self) -> BigFraction {
        let mut dtor: BigInt = BigInt::one();
        let mut result: BigFraction = Self::get_one();
        let mut ntor: BigFraction = self.clone();
        for i in 0..10 {
            dtor = dtor.mul(BigInt::from(i));
            result = result.add(ntor.clone().div_int(dtor.clone()));
            ntor = ntor.clone().mul(self.clone());
        }
        result
    }

    pub fn log(self) -> BigFraction {
        if self.ntor == BigInt::one() && self.dtor == BigInt::one() {
            return Self::get_zero();
        }
        let digits: String = self.to_double().to_string().split(".").next().unwrap().to_string();

        let length: usize;
        if digits.starts_with('0') || digits.starts_with('1') {
            length = digits.len() - 1;
        } else {
            length = digits.len();
        }
        let y: BigFraction = self.div_int(BigInt::from(10).pow(length as u32));
        if y.clone().compare_to(Self::get_two()) <= 0 {
            let mut result: BigFraction = Self::get_zero();
            let x: BigFraction = y.clone().sub(Self::get_one());
            let mut ntor: BigFraction = x.clone();
            let mut dtor: BigInt = BigInt::one();
            let mut sign: BigInt = BigInt::one();
            for _ in 0..200 {
                let temp: BigFraction = ntor.clone().div_int(dtor.clone()).mul_int(sign.clone());
                result = result.add(temp);
                ntor = ntor.clone().mul(x.clone());
                dtor = dtor.clone().add(BigInt::one());
                sign = sign.clone().neg();
            }
            return result.add(Self::get_log_10().mul_int(BigInt::from(length)));
        } else {
            panic!("Unexpected division by largest power of 10")
        }
    }
    pub fn compare_to(self, other: BigFraction) -> i32 {
        return self.ntor.mul(other.dtor).cmp(&other.ntor.mul(self.dtor)) as i32;

    }

    pub fn compare_int_to(self, other: BigInt) -> i32 {
        let other: BigFraction = BigFraction::new(other, BigInt::one()).expect("This should not fail");
        return self.compare_to(other);
    }

    pub fn to_string(&self) -> String {
        if self.dtor == BigInt::one() {
            return self.ntor.to_string();
        }
        return self.ntor.to_string() + "/" + &*self.dtor.to_string();
    }


    pub fn get_denominator(&self) -> BigInt {
        self.dtor.clone()
    }

    pub fn get_numerator(&self) -> BigInt {
        self.ntor.clone()
    }

    pub fn to_double(&self) -> f64 {
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