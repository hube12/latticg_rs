mod math;

use crate::math::big_fraction::BigFraction;
use num_traits::One;


pub fn test() {
    let b= BigFraction::new(One::one(), One::one()).expect("Something");
    dbg!(b);
}