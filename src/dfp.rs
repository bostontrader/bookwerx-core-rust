#[derive(Serialize, Clone, Copy)]
pub struct DFP {
    pub amount: i64,
    pub exp: i8
}

impl DFP {
    pub(crate) fn add(&self, n2: &DFP) -> DFP {
        let d = self.exp - n2.exp;
        if d >= 1 {
            return n2.add(&DFP { amount: self.amount * 10, exp: self.exp - 1 })
        } else if d == 0 {
            return DFP { amount: self.amount + n2.amount, exp: self.exp }
        } else {
            return n2.add(self)
        }
    }
}

#[test]
fn test_dfp() {

    let mut n = DFP{ amount: 1, exp: -1};

    // 1.
    n = DFP { amount: 1, exp: -1}.add(&DFP { amount: 1, exp: -1});
    assert!(n.amount == 2);
    assert!(n.exp == -1);

    // 2.
    n = DFP { amount: 1, exp: -1}.add(&DFP { amount: 1, exp: 0});
    assert!(n.amount == 11);
    assert!(n.exp == -1);

    // 3.
    n = DFP { amount: 1, exp: -1}.add(&DFP { amount: 1, exp: 1});
    assert!(n.amount == 101);
    assert!(n.exp == -1);

    // 4.
    n = DFP { amount: 1, exp: 0}.add(&DFP { amount: 1, exp: -1});
    assert!(n.amount == 11);
    assert!(n.exp == -1);

    // 5.
    n = DFP { amount: 1, exp: 0}.add(&DFP { amount: 1, exp: 0});
    assert!(n.amount == 2);
    assert!(n.exp == 0);

    // 6.
    n = DFP { amount: 1, exp: 0}.add(&DFP { amount: 1, exp: 1});
    assert!(n.amount == 11);
    assert!(n.exp == 0);

    // 7.
    n = DFP { amount: 1, exp: 1}.add(&DFP { amount: 1, exp: -1});
    assert!(n.amount == 101);
    assert!(n.exp == -1);

    // 8.
    n = DFP { amount: 1, exp: 1}.add(&DFP { amount: 1, exp: 0});
    assert!(n.amount == 11);
    assert!(n.exp == 0);

    // 9.
    n = DFP { amount: 1, exp: 1}.add(&DFP { amount: 1, exp: 1});
    assert!(n.amount == 2);
    assert!(n.exp == 1);

}