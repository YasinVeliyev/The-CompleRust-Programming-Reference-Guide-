pub fn add(left: isize, right: isize) -> isize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sums() {
        let data = vec![((1, 1), 2), ((1, 0), 1), ((-1, 1), 0), ((-1, 2), 1)];
        for ((left, right), output) in data {
            assert_eq!(add(left, right), output)
        }
    }
}
