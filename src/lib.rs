mod math;

use crate::math::big_fraction::BigFraction;
use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::borrow::Borrow;


pub fn test() {
    let b= BigFraction::new(One::one(), One::one()).expect("Something");
    dbg!(b);
}