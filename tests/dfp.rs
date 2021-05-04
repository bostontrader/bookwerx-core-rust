use bookwerx_core_rust::dfp::dfp::{DFP, Sign, dfp_abs, dfp_add, dfp_from_string_exp};

#[test]
fn dfp_abs_test() {

    let mut input;
    let mut output;

    input = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    assert_eq!(dfp_abs(&input), output);

    input = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    assert_eq!(dfp_abs(&input), output);

    input = DFP { amount: vec![], exp: 0, sign: Sign::Zero };
    output = DFP { amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!(dfp_abs(&input), output);
}

#[test]
fn dfp_add_test() {

    let mut input1;
    let mut input2;
    let mut output;

    input1 = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    input2 = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    output = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![], exp: 0, sign: Sign::Zero };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![1], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![], exp: 0, sign: Sign::Zero };
    output = DFP{ amount: vec![1], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    //-- Positive + Positive
    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![2], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![3], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![1, 2], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![2, 2], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![2, 1], exp: -1, sign: Sign::Positive };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![2, 4], exp: -1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: -1, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![9, 5], exp: -1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![5], exp: 1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: 1, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![1, 9, 4], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: 1, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: -1, sign: Sign::Positive };
    output = DFP{ amount: vec![1, 0, 9, 4], exp: -1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    //-- Positive + Negative
    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![3], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![2], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![1], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![1,2], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2], exp: 1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1,2], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2], exp: 1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![2,1], exp: -1, sign: Sign::Positive };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![8, 1], exp: -1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: -1, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![9, 3], exp: -1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: 0, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![8, 4], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: 1, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![9, 8, 4], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9,4], exp: 1, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: -1, sign: Sign::Negative };
    output = DFP{ amount: vec![9, 9, 8, 4], exp: -1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    //-- Negative + Positive
    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![2], exp: 0, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![3], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![2], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![1], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1,2], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![2], exp: 1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1, 2], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![2], exp: 1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![2, 1], exp: -1, sign: Sign::Negative };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![8, 1], exp: -1, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: -1, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![9, 3], exp: -1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![8, 4], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: 1, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
    output = DFP{ amount: vec![9, 8, 4], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: 1, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: -1, sign: Sign::Positive };
    output = DFP{ amount: vec![9, 9, 8, 4], exp: -1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    //-- Negative + Negative
    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![4], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![3], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![2], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![5], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1, 2], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2, 2], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![1, 2], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2, 2], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![2, 1], exp: -1, sign: Sign::Negative };
    input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![2, 4], exp: -1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: -1, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![9, 5], exp: -1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: 0, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![5], exp: 1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: 1, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
    output = DFP{ amount: vec![1, 9, 4], exp: 0, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    input1 = DFP { amount: vec![9, 4], exp: 1, sign: Sign::Negative };
    input2 = DFP { amount: vec![1], exp: -1, sign: Sign::Negative };
    output = DFP{ amount: vec![1, 0, 9, 4], exp: -1, sign: Sign::Negative };
    assert_eq!( dfp_add(input1, input2), output);

    // other
    input1 = DFP { amount: vec![9, 0, 0, 0, 0 ,0, 0, 8, 4, 1, 2], exp: -8, sign: Sign::Positive };
    input2 = DFP { amount: vec![1], exp: -8, sign: Sign::Positive };
    output = DFP { amount: vec![1, 0, 0, 0 ,0, 0, 8, 4, 1, 2], exp: -7, sign: Sign::Positive };
    assert_eq!( dfp_add(input1, input2), output);
}

#[test]
fn dfp_from_string_exp_test() {

    let mut output;

    output = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!(dfp_from_string_exp(&("".to_string()), 99), output);

    output = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!(dfp_from_string_exp(&("x".to_string()), 99), output);

    output = DFP{ amount: vec![], exp: 0, sign: Sign::Zero };
    assert_eq!(dfp_from_string_exp(&("-x".to_string()), 99), output);

    output = DFP{ amount: vec![1], exp: 5, sign: Sign::Positive };
    assert_eq!(dfp_from_string_exp(&("1".to_string()), 5), output);

    output = DFP{ amount: vec![2, 1], exp: -5, sign: Sign::Positive };
    assert_eq!(dfp_from_string_exp(&("1x2".to_string()), -5), output);

    output = DFP{ amount: vec![1], exp: 5, sign: Sign::Negative };
    assert_eq!(dfp_from_string_exp(&("-1".to_string()), 5), output);

    output = DFP{ amount: vec![2,9,7,9,8,5,3,5,6,2,9,5,1,4,1,3], exp: 0, sign: Sign::Positive };
    assert_eq!(dfp_from_string_exp(&("3141592653589792".to_string()), 0), output);

    output = DFP{ amount: vec![2,6,7,8,4,0,8,0,8,1,0,2], exp: -8, sign: Sign::Positive };
    assert_eq!(dfp_from_string_exp(&("201808048762".to_string()), -8), output);

    output = DFP{ amount: vec![5], exp: 2, sign: Sign::Positive };
    assert_eq!(dfp_from_string_exp(&("500".to_string()), 0), output);

    output = DFP{ amount: vec![5], exp: 2, sign: Sign::Negative };
    assert_eq!(dfp_from_string_exp(&("-500".to_string()), 0), output);
}
