pub fn and(left: u8, right: u8) -> u8 {
    match (left, right) {
        (1, 1) => 1,
        _ => 0,
    }
}

pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) | (0, 0) => 0,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(and(1, 1), 1);
        assert_eq!(and(0, 1), 0);
        assert_eq!(and(0, 0), 0);
        assert_eq!(and(1, 0), 0);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(1, 0), 1);
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(1, 1), 0);
        assert_eq!(xor(0, 0), 0);
    }
}
