pub mod dfp {

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Sign {
        Positive,
        Negative,
        Zero
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct DFP {
        pub amount : Vec<u8>,
        pub exp : i8,
        pub sign : Sign
    }

    /* Given a &DFP return a new DFP that is the absolute value of the original. */
    pub fn dfp_abs(dfp :&DFP) -> DFP {

        let mut ret_val = (*dfp).clone();
        if (*dfp).sign == Sign::Negative {
            ret_val.sign = Sign::Positive;
        } // else if sign == Positive or Zero, do not change it.
        ret_val
    }

    /* Given two DFP, add them together and return a new DFP as the sum. */
    pub fn dfp_add( dfp1 : DFP, dfp2 : DFP) -> DFP {

        // Both of these now have the same exp.  We can freely use n1.exp to build a return value with
        // the correct exp.
        let (n1, n2) = dfp_norm_exp( dfp1, dfp2);

        let ret_val =
            match (&n1.sign, &n2.sign) {

                // If one DFP is zero, return the other one.
                (Sign::Zero, _) => n2.clone(),
                (_, Sign::Zero) => n1.clone(),

                // If both arguments are Positive just add them for a Positive sum.
                (Sign::Positive, Sign::Positive) => DFP {
                    //amount: md_add( n1.amount, n2.amount, 0), exp: n1.exp, sign: Sign::Positive
                    amount: md_add( n1.amount, n2.amount), exp: n1.exp, sign: Sign::Positive
                },

                // If both arguments are Negative return - (|a| + |b|)
                (Sign::Negative, Sign::Negative) => DFP {
                    //amount: md_add( n1.amount, n2.amount, 0 ), exp: n1.exp, sign: Sign::Negative
                    amount: md_add( n1.amount, n2.amount), exp: n1.exp, sign: Sign::Negative
                },

                // If the signs of the arguments are different...
                (Sign::Positive, Sign::Negative) => {
                    dfp_add1(n1, n2)
                },

                (Sign::Negative, Sign::Positive) => {
                    dfp_add1(n2, n1)
                },

            };
        dfp_norm(ret_val)
    }

    /* Given two DFP of different signs, return their sum.  This is a special purpose function
    that exists solely to support dfp_add and has no wider general purpose use.  The reasoning
    for this function's existence is that the addition of two DFP with different signs is required
    more than once in dfp_add and is thus factored out here.
     */
    fn dfp_add1( n1 : DFP, n2 : DFP) -> DFP {

        // compare |n1| and |n2|
        let cmp = md_compare(&n1.amount, &n2.amount);

        // If |n1| == |n2|, given that the signs are different, these add up to zero.
        if cmp == 0 {
            return DFP { amount: vec![], exp: 0, sign: Sign::Zero }
        };

        // |n1| != |n2| so one of them must be the larger one.  We to return a DFP that contains the difference between
        // |larger| - |smaller| with a suitable sign.

        if cmp == -1 {
            let diff = md_sub(&n2.amount, &n1.amount);
            DFP { amount: *diff, exp: n1.exp, sign: Sign::Negative }
        } else { // cmp > 0
            let diff = md_sub(&n1.amount, &n2.amount);
            DFP { amount: *diff, exp: n1.exp, sign: Sign::Positive }
        }
    }

    /*
    Normalize a DFP. Given a DFP A, return a new DFP B that represents the same number such that the significand of DFP B has no LSD zeros.

    For example: [1], 3, [0,1], 2 and [0,0,1], 1 all represent 1000, but let's use the first choice as the normalized value.
    */
    fn dfp_norm( dfp : DFP) -> DFP {
        let ln = dfp.amount.len();
        if ln == 0 {
            DFP { amount: vec![], exp: 0, sign: Sign::Zero }
        } else if ln == 1 {
            if dfp.amount[0] == 0 {
                DFP { amount: vec![], exp: 0, sign: Sign::Zero }
            } else {
                dfp.clone()
            }
        } else { // ln > 1
            if dfp.amount[0] == 0 {
                let mut v1 = dfp.amount.clone();
                let _ = v1.remove(0);
                let new_dfp = DFP { amount: v1, exp: dfp.exp + 1, sign: dfp.sign };
                dfp_norm( new_dfp)
            } else {
                dfp.clone()
            }
        }
    }

    #[test]
    fn dfp_norm_test() {

        let mut input;
        let mut output;

        input = DFP { amount: vec![], exp: 0, sign: Sign::Positive };
        output = DFP { amount: vec![], exp: 0, sign: Sign::Zero };
        assert_eq!(dfp_norm(input), output);

        input = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
        output = DFP { amount: vec![1], exp: 0, sign: Sign::Positive };
        assert_eq!(dfp_norm(input), output);

        input = DFP { amount: vec![0, 1], exp: 0, sign: Sign::Positive };
        output = DFP { amount: vec![1], exp: 1, sign: Sign::Positive };
        assert_eq!(dfp_norm(input), output);
    }

    /*
    Suppose you want to add 1 + 0.1.  As DFP they'd be represented as [1] 0 Positive and [1] -1 Positive.  In
    order to perform the addition we'll first have to change [1] 0 to [0, 1] -1.  This is still the same number
    but now the exponents of both DFP match and we can simply add the digits normally.

    This function takes two DFP n1, and n2.  If the exponents are the same it merely
    returns (n1, n2).  But if the exponents are different, it adjusts the amount and exponent of the DFP with the
    higher exponent by successively appending a 0 as a LSD and decrementing the exponent until the exponent matches
    the other DFP.  This function then returns (new n1, new n2).
    */
    fn dfp_norm_exp( n1 : DFP, n2 : DFP) -> (DFP, DFP) {
        if n1.exp == n2.exp {
            (n1, n2)
        } else if n1.exp > n2.exp {
            let mut v1 :Vec<u8> = vec![0];
            let mut v2 :Vec<u8> = n1.amount.clone();
            v1.append(&mut v2);

            let new_n1 = DFP { amount: v1, exp: n1.exp - 1, sign: n1.sign };
            dfp_norm_exp( new_n1, n2)
        } else {
            let mut v1 :Vec<u8> = vec![0];
            let mut v2 :Vec<u8> = n2.amount.clone();
            v1.append(&mut v2);

            let new_n2 = DFP { amount: v1, exp: n2.exp - 1, sign: n2.sign };
            dfp_norm_exp( n1, new_n2)
        }
    }

    #[test]
    fn dfp_norm_exp_test() {

        let mut input1;
        let mut input2;
        let mut output1;
        let mut output2;

        // n1.exp == n2.exp
        input1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
        input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
        output1 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
        output2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
        assert_eq!((output1, output2),dfp_norm_exp(input1,input2));

        // n1.exp > n2.exp
        input1 = DFP { amount: vec![1], exp: 1, sign: Sign::Positive };
        input2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
        output1 = DFP { amount: vec![0,1], exp: 0, sign: Sign::Positive };
        output2 = DFP { amount: vec![1], exp: 0, sign: Sign::Negative };
        assert_eq!((output1, output2),dfp_norm_exp(input1,input2));

        // n1.exp < n2.exp
        input1 = DFP { amount: vec![2], exp: 0, sign: Sign::Negative };
        input2 = DFP { amount: vec![1], exp: 1, sign: Sign::Positive };
        output1 = DFP { amount: vec![2], exp: 0, sign: Sign::Negative };
        output2 = DFP { amount: vec![0,1], exp: 0, sign: Sign::Positive };
        assert_eq!((output1, output2),dfp_norm_exp(input1,input2));

        // 1.2, 3
        input1 = DFP { amount: vec![2, 1], exp: -1, sign: Sign::Positive };
        input2 = DFP { amount: vec![3], exp: 0, sign: Sign::Positive };
        output1 = DFP { amount: vec![2, 1], exp: -1, sign: Sign::Positive };
        output2 = DFP { amount: vec![0, 3], exp: -1, sign: Sign::Positive };
        assert_eq!((output1, output2),dfp_norm_exp(input1,input2));
    }

    pub fn dfp_from_string_exp(s :&String, exp :i8) -> DFP {

        // 1. Given the input s, produce (neg_sign :bool, s1 :&String) where neg_sign ==
        // true if s starts with "-" or otherwise false, and s1 is the remaining s, after
        // said "-" is removed, if any.
        let l = s.len();

        let n1 = if l == 0 {
            ""
        } else {
            &s[0..1]
        };

        let (neg_sign, s1) = if n1 == "-" {
            (true, s)
        } else {
            (false, s)
        };

        // 2. Build a filter to remove all non-numeric characters from s1
        let s2 = s1.chars().filter(|x|
            match *x {
                '0' => true,
                '1' => true,
                '2' => true,
                '3' => true,
                '4' => true,
                '5' => true,
                '6' => true,
                '7' => true,
                '8' => true,
                '9' => true,
                _ => false,
            }
        );

        // 3. Build a map to convert all numeric char from s2 to u8
        let s3 = s2.map(|c| -> u8 {
            match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => 0
            }
        });

        // 4. Invoke the filter and map producing a Vec<u8> of the digits we want, in the correct order.
        let s4 :Vec<u8> = s3.rev().collect();
        let s4_len = s4.len();

        // 5. Produce the final result, normalize it, and return it.
        let ret_val = if s4_len == 0 {
            DFP { amount: vec![], exp: 0, sign: Sign::Zero }
        } else if neg_sign {
            DFP { amount: s4, exp: exp, sign: Sign::Negative }
        } else {
            DFP { amount: s4, exp: exp, sign: Sign::Positive }
        };

        dfp_norm( ret_val )

    }

    // Given two integers of a given base, and a prior carry (or zero if none) calculate their sum modulo base and return (sum, carry)
    fn dnc( i1 :u8, i2 :u8, carry :u8, base :u8) -> (u8, u8) {
        let sm = i1 + i2 + carry;
        (sm % base, sm / base)
    }

    #[test]
    fn dnc_test() {
        assert_eq!((0,0), dnc(0,0,0,10));
        assert_eq!((1,0), dnc(0,1,0,10));
        assert_eq!((1,1), dnc(5,6,0,10));
        assert_eq!((8,19), dnc(99,99,0,10));
    }

    /* Given two Vec<u8> perform a multi-digit addition of them and return
    the resulting Vec<u8> sum.
    */
    fn md_add( v1 :Vec<u8>, v2 :Vec<u8>) -> Vec<u8> {
        let ln1 = v1.len();
        let ln2 = v2.len();
        let mut ret_val :Vec<u8> = Vec::new();

        // 1. The loop that adds these digits will want to iterate over all digits of
        // the longest vec,or either vec if they are the same length.  Start by assuming
        // that ln1 >= ln2 and set these variables accordingly.
        let mut long_vec :&Vec<u8> = &v1;
        let mut short_vec :&Vec<u8> = &v2;
        let mut long_len = ln1;
        let mut short_len = ln2;

        // 2. If our prior assumption is wrong, then reset the variables.
        if ln1 < ln2 {
            long_vec = &v2;
            short_vec = &v1;
            long_len = ln2;
            short_len = ln1;
        }

        // 3. If long_len == 0 then we know that both vec are empty and that the result is also empty.
        if long_len == 0 {
            return ret_val
        }

        // 4. Now add the digits.
        let mut carry :u8 = 0;
        for i in 0..long_len {
            if i >= short_len {
                let (s, c) = dnc( long_vec[i], 0, carry, 10);
                ret_val.push(s);
                carry = c;
            } else {
                let (s, c) = dnc( long_vec[i], short_vec[i], carry, 10);
                ret_val.push(s);
                carry = c;
            }
        };

        // 5. After the smoke clears, if there's still a carry, that's the MSD of the final answer.
        if carry != 0 {
            ret_val.push(carry);
        }

        ret_val
    }

    #[test]
    fn md_add_test() {
        let mut input1;
        let mut input2;
        let mut output :Vec<u8>;

        input1 = vec![];
        input2 = vec![];
        output = vec![];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![];
        input2 = vec![1];
        output = vec![1];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![];
        input2 = vec![1, 2, 3];
        output = vec![1, 2, 3];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![1];
        input2 = vec![];
        output = vec![1];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![1, 2, 3];
        input2 = vec![];
        output = vec![1, 2, 3];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![1];
        input2 = vec![1];
        output = vec![2];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![5];
        input2 = vec![6];
        output = vec![1, 1];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![5, 5];
        input2 = vec![9];
        output = vec![4, 6];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![5, 5];
        input2 = vec![6, 6];
        output = vec![1, 2, 1];
        assert_eq!( md_add( input1, input2), output);

        input1 = vec![6, 1, 5];
        input2 = vec![1, 6, 7];
        output = vec![7, 7, 2, 1];
        assert_eq!( md_add( input1, input2), output);
    }

    /* Given two Vec<u8> v1 and v2 return -1 if v1 < v2, 0 if v1 == v2, or 1 if v1 > v2.
    */

    fn md_compare( v1 : &Vec<u8>, v2 : &Vec<u8> ) -> i8 {

        let ln1 = v1.len();
        let ln2 = v2.len();

        // 1. If the lengths are different we automatically know that one v > than other
        if ln1 < ln2 {
            return -1
        } else if ln1 > ln2 {
            return 1
        };

        // 2. We know the lenghts are the same. If they are both 0 then the v equal.
        if ln1 == 0 { return 0 }

        // 3. Compare the digits from MSD to LSD.
        for i in (0..ln1).rev() {

            if v1[i] < v2[i] {
                return -1
            } else if v1[i] > v2[i] {
                return 1
            }
        };

        0 // Now we know that the v are equal

    }

    #[test]
    fn md_compare_test() {

        let mut input1;
        let mut input2;

        input1 = vec![];
        input2 = vec![];
        assert_eq!( md_compare( &input1, &input2 ), 0);

        input1 = vec![1];
        input2 = vec![];
        assert_eq!( md_compare( &input1, &input2 ), 1);

        input1 = vec![];
        input2 = vec![1];
        assert_eq!( md_compare( &input1, &input2 ), -1);

        input1 = vec![1];
        input2 = vec![1];
        assert_eq!( md_compare( &input1, &input2 ), 0);

        input1 = vec![1];
        input2 = vec![2];
        assert_eq!( md_compare( &input1, &input2 ), -1);

        input1 = vec![2];
        input2 = vec![1];
        assert_eq!( md_compare( &input1, &input2 ), 1);

        input1 = vec![1, 2];
        input2 = vec![1, 3];
        assert_eq!( md_compare( &input1, &input2 ), -1);

        input1 = vec![1, 2];
        input2 = vec![1, 2];
        assert_eq!( md_compare( &input1, &input2 ), 0);

        input1 = vec![1, 2];
        input2 = vec![1, 1];
        assert_eq!( md_compare( &input1, &input2 ), 1);

    }

    // Given a Vec<u8>, remove all msd zeros, if any.
    fn md_stripz( v :&Vec<u8>) -> Vec<u8> {

        let mut ret_val = v.clone();
        let mut len = ret_val.len();
        while len > 0 && ret_val[len-1] == 0 {
            let _ = ret_val.remove(len-1);
            len = len - 1;
        };

        ret_val
    }

    #[test]
    fn md_stripz_test() {

        let mut input;
        let mut output;

        input = vec![];
        output = vec![];
        assert_eq!(md_stripz(&input), output);

        input = vec![0];
        output = vec![];
        assert_eq!(md_stripz(&input), output);

        input = vec![1];
        output = vec![1];
        assert_eq!(md_stripz(&input), output);

        input = vec![1, 0];
        output = vec![1];
        assert_eq!(md_stripz(&input), output);

        input = vec![0, 1, 0, 0];
        output = vec![0, 1];
        assert_eq!(md_stripz(&input), output);

        input = vec![0, 1];
        output = vec![0, 1];
        assert_eq!(md_stripz(&input), output);

    }

    /*
    Given two Vec<u8>, top and bottom,  perform a multi-digit subtraction of bottom from top and return Maybe (the resulting value).  This is a Maybe because parameter errors are possible.

    This is the public entry point where obvious cases are dealt with.

    This function uses the "Three Digit Trick" (http://web.sonoma.edu/users/w/wilsonst/courses/math_300/groupwork/altsub/3d.html) generalized for Vec of digits of indefinite length.

    The advantage of this method is that we don't have to deal with regrouping.  The disadvantage is that there are many gyrations required to get this done making this somewhat more difficult to understand and probably impacting running time.

    This function is a very special purpose thing that exists solely to support dfp_add.  dfp_add needs subtraction in order to deal with negative DFP numbers.  As such, this function only cares about doing that and so it has the following rather tedious constraints:

    * top and bottom are both Vec of "small" positive integers from 0 to base-1 inclusive and have not been tested with "large" or negative integers.

    * the top and bottom Vec both represent positive numbers.  In the event of the necessity of subtraction of a negative number, re-arrange the problem using negation such that this constraint can hold.

    * top >= bottom
     */
    fn md_sub(top :&Vec<u8>, bottom :&Vec<u8>) -> Box<Vec<u8>> {
        let zz_top = md_stripz(top);
        let zz_bot = md_stripz( bottom );
        let lnt = zz_top.len();
        let lnb = zz_bot.len();
        if lnt == 0 && lnb == 0 {
            return Box::new(vec![]);
        } else if lnt == 1 && lnb == 0 {
            return Box::new(zz_top.clone());
        } else if lnt == 1 && lnb == 1 {
            return Box::new(vec![ zz_top[0] - zz_bot[0]]);
        } else if lnt > 1 && lnb == 0 {
            return Box::new(zz_top.clone());
        } else if lnt > 1 && lnb == 1 {
            return md_sub1( zz_top, zz_bot);
        } else if lnt > 1 && lnb > 1 {
            return md_sub1( zz_top, zz_bot);
        };
        return Box::new(vec![]);
    }

    /*
    Given two Vec<u8>, top and bottom,  perform a multi-digit subtraction of bottom from top and return
    a Box<Vec<u8>> containing the resulting value.

    This is a private method that performs the actual subtraction for md_sub which is
    responsible for any error checking.  md_sub is also responsible for ensuring that top > bottom when
    invoking this function.
    */
    fn md_sub1(top :Vec<u8>, bottom :Vec<u8>) -> Box<Vec<u8>> {

        let n1 = *md_sub2(bottom, top.len() as u8);
        let mut n2 = md_add(top,n1);
        let l2 = (*n2).len();

        n2[l2-1] = n2[l2-1]-1; // subtract 1000

        let n3 = md_add( vec![1], n2);
        Box::new(md_stripz(&n3))
    }

    /*
    In order to perform a subtraction using the selected algorithm, we want to subtract
    each digit of the bottom number from 9.

    This function will create a new Vec to hold the answer.  We will return said Vec in
    a Box.
     */
    fn md_sub2( bottom :Vec<u8>, len :u8) -> Box<Vec<u8>> {

        let mut ret_val :Vec<u8> = Vec::new();
        let lnb :u8 = bottom.len() as u8;
        if len == 0 {
            Box::new(ret_val)
        } else {
            for i in 0..len {
                if i < lnb {
                    ret_val.push(9 - bottom[i as usize]);
                } else {
                    ret_val.push(9 );
                }
            }
            Box::new(ret_val)
        }

    }


    #[test]
    fn md_sub_test() {

        let mut input1;
        let mut input2;
        let mut output;

        input1 = vec![0];
        input2 = vec![0];
        output = vec![];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![];
        input2 = vec![];
        output = vec![];
        assert_eq!( *md_sub(&input1, &input2), output);

        //-- should never happen because top sb >= bottom
        //--, test "[] + [1]" (\_ -> Expect.equal (md_sub [] [1] 0 10 ) ([1]))
        //--, test "[] + [1, 2, 3]" (\_ -> Expect.equal (md_sub [] [1, 2, 3] 0 10 ) ([1,2,3]))

        input1 = vec![1];
        input2 = vec![];
        output = vec![1];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![1, 0];
        input2 = vec![0];
        output = vec![1];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![1, 2, 3];
        input2 = vec![];
        output = vec![1, 2, 3];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![6, 1, 5];
        input2 = vec![8, 3, 2];
        output = vec![8, 7, 2];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![3];
        input2 = vec![1];
        output = vec![2];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![3];
        input2 = vec![1, 0];
        output = vec![2];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![5, 5];
        input2 = vec![9];
        output = vec![6, 4];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![5, 5];
        input2 = vec![6, 4];
        output = vec![9];
        assert_eq!( *md_sub(&input1, &input2), output);

        input1 = vec![0, 9, 4];
        input2 = vec![1];
        output = vec![9, 8, 4];
        assert_eq!( *md_sub(&input1, &input2), output);

    }

}