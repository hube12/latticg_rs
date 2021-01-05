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
    let res = a.div(b);
    assert!(res.is_some());
    assert_eq!(c, res.unwrap());
}

#[test]
fn test_div_fraction_with_integer() {
    let a: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(11)).expect("Correct");
    let c: BigFraction = BigFraction::new(BigInt::from(7), BigInt::from(143)).expect("Correct");
    let res = a.div_int(BigInt::from(13));
    assert!(res.is_some());
    assert_eq!(c, res.unwrap());
}

#[test]
fn test_division_by_zero() {
    let res = BigFraction::get_one().div(BigFraction::get_zero());
    assert!(res.is_none())
}

#[test]
fn test_negate_zero() {
    assert_eq!(BigFraction::get_zero().negate(), BigFraction::get_zero());
}

#[test]
fn test_negate_positive() {
    assert_eq!(BigFraction::get_minus_one(), BigFraction::get_one().negate());
}

#[test]
fn test_negate_negative() {
    assert_eq!(BigFraction::get_minus_one().negate(), BigFraction::get_one());
}

#[test]
fn test_reciprocal() {
    assert_eq!(BigFraction::get_half(), BigFraction::get_two().reciprocal().expect("Should work"));
}

#[test]
fn test_reciprocal_zero() {
    assert!(BigFraction::get_zero().reciprocal().is_none());
}

#[test]
fn test_floor_zero() {
    assert_eq!(BigFraction::get_zero().floor(), BigInt::zero());
}

#[test]
fn test_floor_positive_integer() {
    assert_eq!(BigFraction::get_one().floor(), BigInt::one())
}

#[test]
fn test_floor_negative_integer() {
    assert_eq!(BigFraction::get_minus_one().floor(), -BigInt::one())
}

#[test]
fn test_floor_positive() {
    assert_eq!(BigFraction::new(BigInt::from(3), BigInt::from(2)).unwrap().floor(), BigInt::one());
}


#[test]
fn test_floor_negative() {
    assert_eq!(BigFraction::new(BigInt::from(-3), BigInt::from(2)).unwrap().floor(), BigInt::from(-2));
}

#[test]
fn test_ceil_zero() {
    assert_eq!(BigFraction::get_zero().ceil(), BigInt::zero());
}

#[test]
fn test_ceil_positive_integer() {
    assert_eq!(BigFraction::get_one().ceil(), BigInt::one());
}

#[test]
fn test_ceil_negative_integer() {
    assert_eq!(BigFraction::get_minus_one().ceil(), -BigInt::one());
}

#[test]
fn test_ceil_positive() {
    assert_eq!(BigFraction::new(BigInt::from(3), BigInt::from(2)).unwrap().ceil(), BigInt::from(2));
}

#[test]
fn test_ceil_negative() {
    assert_eq!(BigFraction::new(BigInt::from(-3), BigInt::from(2)).unwrap().ceil(), BigInt::from(-1));
}

#[test]
fn test_round_zero() {
    assert_eq!(BigFraction::get_zero().round(), BigInt::zero());
}

#[test]
fn test_round_positive_integer() {
    assert_eq!(BigFraction::get_one().round(), BigInt::one());
}

#[test]
fn test_round_negative_integer() {
    assert_eq!(BigFraction::get_minus_one().round(), -BigInt::one());
}

#[test]
fn test_round_half() {
    assert_eq!(BigFraction::get_half().round(), BigInt::one());
}

#[test]
fn test_round_minus_half() {
    assert_eq!(BigFraction::get_half().negate().round(), BigInt::zero());
}

#[test]
fn test_rounding_less_than_half_positive() {
    assert_eq!(BigFraction::new(BigInt::from(4), BigInt::from(3)).unwrap().round(), BigInt::from(1));
}

#[test]
fn test_rounding_more_than_half_positive() {
    assert_eq!(BigFraction::new(BigInt::from(5), BigInt::from(3)).unwrap().round(), BigInt::from(2));
}

#[test]
fn test_rounding_less_than_half_negative() {
    assert_eq!(BigFraction::new(BigInt::from(-5), BigInt::from(3)).unwrap().round(), BigInt::from(-2));
}

#[test]
fn test_signum_zero() {
    assert_eq!(BigFraction::get_zero().signum(), BigInt::zero());
}

#[test]
fn test_signum_positive() {
    assert_eq!(BigFraction::get_half().signum(), BigInt::one());
}

#[test]
fn test_signum_negative() {
    assert_eq!(BigFraction::get_half().negate().signum(), -BigInt::one());
}

#[test]
fn test_abs_zero() {
    assert_eq!(BigFraction::get_zero().abs(), BigFraction::get_zero());
}

#[test]
fn test_abs_positive() {
    assert_eq!(BigFraction::get_one().abs(), BigFraction::get_one());
}

#[test]
fn test_abs_negative() {
    assert_eq!(BigFraction::get_minus_one().abs(), BigFraction::get_one());
}

#[test]
fn test_compare_equal() {
    assert_eq!(BigFraction::get_one().compare_to(BigFraction::get_one()), 0);
}

#[test]
fn test_compare_int_equal() {
    assert_eq!(BigFraction::get_one().compare_int_to(BigInt::one()), 0);
}


#[test]
fn test_compare() {
    let a = BigFraction::new(BigInt::from(7), BigInt::from(11)).unwrap();
    let b = BigFraction::new(BigInt::from(13), BigInt::from(17)).unwrap();
    assert!(a.clone().compare_to(b.clone()) < 0);
    assert!(b.clone().compare_to(a.clone()) > 0);
}


#[test]
fn test_exp_small() {
    let a = BigFraction::new(BigInt::from(42), BigInt::from(4242)).unwrap();
    let res_a = BigFraction::parse("40082549934887931699451/39687651175170088375488".parse().unwrap()).expect("Can not fail");
    let a_exp = a.exp();
    assert_eq!(res_a, a_exp);
    assert_eq!(1.0099501670677076, a_exp.to_double());
}

#[test]
fn test_exp_big() {
    let a = BigFraction::new(BigInt::from(42424), BigInt::from(1)).unwrap();
    let res_a = BigFraction::parse("3478457471619510415813461337374760474907/2835".parse().unwrap()).expect("Can not fail");
    let a_exp = a.exp();

    assert_eq!(1.2269691257917144E36, a_exp.to_double());
    assert_eq!(res_a, a_exp);
}

#[test]
fn test_log_small() {
    let a = BigFraction::new(BigInt::from(42), BigInt::from(4242)).unwrap();
    let res_a = BigFraction::parse("-9960205191509052037635093850393521875699378710900704375320890078231380448345221311534733915424354924668747282724966432825732316923508496866772998109861630312319402851616062268928222911733380529013299181810430569435131017695276801886814339738969890153826225586051519071902129849320972224210244121592516790125268151193048280845001772832473348568191009972574261781655247162007164087796292386292758995430599195099757003613631365910090947751711235384515182197955661816334263665401922659100/2181440874477967117112307434452612853292909564778615037490592307074840359220917034882712432622114313078888260277096245688026245487332109797997656100116029427210272800247173265580922078270745982498254924219461809708743149532972945863478493999269309380359699908638416051732364944781175136855447542856910864773401654799402642742471320837932575913308438099591358408320209829547753287962769968026162574688490276863801797221417212644836695259932121527784940694224306223148600390561610938641".parse().unwrap()).expect("Can not fail");
    let a_log = a.log();
    assert_eq!(-4.565883635921415, a_log.to_double());
    assert_eq!(res_a, a_log);
}

#[test]
fn test_log_big() {
    let a = BigFraction::new(BigInt::from(42424), BigInt::from(1)).unwrap();
    let res_a = BigFraction::parse("17733211102954688210629798345892516067041612008869292494159499219951390290001807876563109754316896698028488894923639275707380110997840301465173462791847051248013808760277459255791659653317955910543614472125096556981007893822270664313549205043928347804290335652180691334807682064505344142345725220182742850129812680917697621089456815225511197800849863611057699712699884654489622435966347932642777331046113876874987056236795999046148286740664172408359656585634475066762519935080714067379784810939038502837629498456039007045564536846877241001080841986017351188887419891855871681708653427351835134687387400601044496359503346238104396912740271448213476529477091235395680881807440244960105271776380912312395734205518723774979730693386044234476327817697351982152245749154213460824643368213593778795177922746115843527234321365539333214889604297295079687685738256714157654910011031054401287491225559155494847989055098563883157579/1664235543223958804431814781828264855240430819382531307663141387924406749317782419933635463164918917203060647863546407171074576170062923832952183917174563950339656230007798481906606919806741597969324901103463524277608582048833066087813158399479350812421106305231724856670958945573048263290919563419640418623811473223324705815088700044241686705666429367282827602918359556416841188868047484765889079617499574940885383055476725613496621522819033235841715544846371227657894515403538826525675631273770704865455627441406250000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".parse().unwrap()).expect("not to fail");
    let a_log = a.log();
    assert_eq!(10.655469518816966, a_log.to_double());
    assert_eq!(res_a, a_log);
}

#[test]
fn test_log_1() {
    let a = BigFraction::new(BigInt::from(1), BigInt::from(1)).unwrap();
    let res_a = BigFraction::parse("0/1".parse().unwrap()).expect("not to fail");
    let a_log = a.log();
    assert_eq!(0f64, a_log.to_double());
    assert_eq!(res_a, a_log);
}


