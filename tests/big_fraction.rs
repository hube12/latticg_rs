use num_bigint::BigInt;
use num_traits::{One, Zero};
use latticg::math::big_fraction::{BigFraction, ParseBigFractionError};

#[test]
fn test_big_fraction(){
    let one: Result<BigFraction, ParseBigFractionError> =BigFraction::new(BigInt::one(), BigInt::one());
    assert!(one.is_ok());
    assert_eq!(one.clone().unwrap().get_numerator(), BigInt::one());
    assert_eq!(one.clone().unwrap().get_denominator(), BigInt::one());
}

#[test]
fn test_construct_divide_zero(){
    let zero: Result<BigFraction, ParseBigFractionError> =BigFraction::new(BigInt::one(), BigInt::zero());
    assert!(zero.is_err());
    assert_eq!(zero.unwrap_err().to_string(),"Dividing by zero");
}


#[test]
fn test_construct_negative_denominator1(){
    let minus_half: Result<BigFraction, ParseBigFractionError> =BigFraction::new(BigInt::from(1), BigInt::from(-2));
    assert!(minus_half.is_ok());
    assert_eq!(minus_half.clone().unwrap().get_numerator(), BigInt::from(-1));
    assert_eq!(minus_half.clone().unwrap().get_denominator(), BigInt::from(2));
}

#[test]
fn test_construct_negative_denominator2(){
    let half: Result<BigFraction, ParseBigFractionError> =BigFraction::new(BigInt::from(-1), BigInt::from(-2));
    assert!(half.is_ok());
    assert_eq!(half.clone().unwrap().get_numerator(), BigInt::from(1));
    assert_eq!(half.clone().unwrap().get_denominator(), BigInt::from(2));
}

#[test]
fn test_construct_simplify(){
    let half: Result<BigFraction, ParseBigFractionError> =BigFraction::new(BigInt::from(2), BigInt::from(4));
    assert!(half.is_ok());
    assert_eq!(half.clone().unwrap().get_numerator(), BigInt::from(1));
    assert_eq!(half.clone().unwrap().get_denominator(), BigInt::from(2));
}

#[test]
fn test_to_double(){
    let half: Result<BigFraction, ParseBigFractionError> =BigFraction::new(BigInt::from(1), BigInt::from(2));
    assert!(half.is_ok());
    let half:BigFraction=half.unwrap();
    let half2:BigFraction=BigFraction::get_half();
    assert_eq!(half.to_double(),0.5f64);
    assert_eq!(half2.to_double(),0.5f64);
}
