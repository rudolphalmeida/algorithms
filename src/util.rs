#[inline]
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    (1..arr.len()).all(|i| arr[i - 1] < arr[i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        let v = vec![5, 4, 3, 2, 1];
        assert!(!is_sorted(&v));

        let v = vec![1, 2, 3, 4, 5];
        assert!(is_sorted(&v));
    }
}
