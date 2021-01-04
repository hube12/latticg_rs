use num_bigint::BigInt;
use num_traits::{One, Zero};
use latticg::math::big_fraction::{BigFraction, ParseBigFractionError};

#[test]
fn test_big_fraction() {
    let one: Result<BigFraction, ParseBigFractionError> = BigFraction::new(BigInt::one(), BigInt::one());
    assert!(one.is_ok());
    assert_eq!(one.clone().unwrap().get_numerator(), BigInt::one());
    assert_eq!(one.clone().unwrap().get_denominator(), BigInt::one());
}

#[test]
fn test_construct_divide_zero() {
    let zero: Result<BigFraction, ParseBigFractionError> = BigFraction::new(BigInt::one(), BigInt::zero());
    assert!(zero.is_err());
    assert_eq!(zero.unwrap_err().to_string(), "Dividing by zero");
}


#[test]
fn test_construct_negative_denominator1() {
    let minus_half: Result<BigFraction, ParseBigFractionError> = BigFraction::new(BigInt::from(1), BigInt::from(-2));
    assert!(minus_half.is_ok());
    assert_eq!(minus_half.clone().unwrap().get_numerator(), BigInt::from(-1));
    assert_eq!(minus_half.clone().unwrap().get_denominator(), BigInt::from(2));
}

#[test]
fn test_construct_negative_denominator2() {
    let half: Result<BigFraction, ParseBigFractionError> = BigFraction::new(BigInt::from(-1), BigInt::from(-2));
    assert!(half.is_ok());
    assert_eq!(half.clone().unwrap().get_numerator(), BigInt::from(1));
    assert_eq!(half.clone().unwrap().get_denominator(), BigInt::from(2));
}

#[test]
fn test_construct_simplify() {
    let half: Result<BigFraction, ParseBigFractionError> = BigFraction::new(BigInt::from(2), BigInt::from(4));
    assert!(half.is_ok());
    assert_eq!(half.clone().unwrap().get_numerator(), BigInt::from(1));
    assert_eq!(half.clone().unwrap().get_denominator(), BigInt::from(2));
}

#[test]
fn test_to_double() {
    let half: Result<BigFraction, ParseBigFractionError> = BigFraction::new(BigInt::from(1), BigInt::from(2));
    assert!(half.is_ok());
    let half: BigFraction = half.unwrap();
    let half2: BigFraction = BigFraction::get_half();
    assert_eq!(half.to_double(), 0.5f64);
    assert_eq!(half2.to_double(), 0.5f64);
}


#[test]
fn test_add_fraction_with_fraction() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let b: BigFraction = BigFraction::new(BigInt::from(13), BigInt::from(17)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(262), BigInt::from(187)).expect("Correct");
    assert_eq!(c, a.add(b));
}

#[test]
fn test_add_fraction_with_integer() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(150), BigInt::from(11)).expect("Correct");
    assert_eq!(c, a.add_int(BigInt::from(13)));
}


#[test]
fn test_sub_fraction_with_fraction() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let b: BigFraction = BigFraction::new(BigInt::from(13), BigInt::from(17)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(-24), BigInt::from(187)).expect("Correct");
    assert_eq!(c, a.sub(b));
}

#[test]
fn test_sub_fraction_with_integer() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(-136), BigInt::from(11)).expect("Correct");
    assert_eq!(c, a.sub_int(BigInt::from(13)));
}

#[test]
fn test_mul_fraction_with_fraction() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let b: BigFraction = BigFraction::new(BigInt::from(13), BigInt::from(17)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(91), BigInt::from(187)).expect("Correct");
    assert_eq!(c, a.mul(b));
}

#[test]
fn test_mul_fraction_with_integer() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(91), BigInt::from(11)).expect("Correct");
    assert_eq!(c, a.mul_int(BigInt::from(13)));
}

#[test]
fn test_div_fraction_with_fraction() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let b: BigFraction = BigFraction::new(BigInt::from(13), BigInt::from(17)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(119), BigInt::from(143)).expect("Correct");
    let res=a.div(b);
    assert!(res.is_some());
    assert_eq!(c,res.unwrap() );
}

#[test]
fn test_div_fraction_with_integer() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(143)).expect("Correct");
    let res=a.div_int(BigInt::from(13));
    assert!(res.is_some());
    assert_eq!(c,res.unwrap() );
}

#[test]
fn test_division_by_zero(){
    let res=BigFraction::get_one().div(BigFraction::get_zero());
    assert!(res.is_none())
}
#[test]
fn test_negate_zero(){
    assert_eq!(BigFraction::get_zero().negate(),BigFraction::get_zero());
}

#[test]
fn test_negate_positive(){
    assert_eq!(BigFraction::get_minus_one(),BigFraction::get_one().negate());
}

#[test]
fn test_negate_negative(){
    assert_eq!(BigFraction::get_minus_one().negate(),BigFraction::get_one());
}

#[test]
fn test_reciprocal(){
    assert_eq!(BigFraction::get_half(),BigFraction::get_two().reciprocal().expect("Should work"));
}

#[test]
fn test_reciprocal_zero(){
    assert!(BigFraction::get_zero().reciprocal().is_none());
}