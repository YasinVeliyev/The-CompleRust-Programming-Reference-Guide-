use logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

pub fn half_adder_in_out() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    for (inp, out) in half_adder_in_out() {
        let (a, b) = inp;
        println!("Testing: {}, {} -> {:?}", a, b, out);
        assert_eq!(half_adder(a, b), out)
    }
}
