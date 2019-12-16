use std::cmp::Ordering;

use crate::util::is_sorted;

pub fn linear_search<T: Eq, I: AsRef<[T]>>(arr: I, element: &T) -> Option<usize> {
    for (i, value) in arr.as_ref().iter().enumerate() {
        if value == element {
            return Some(i);
        }
    }

    None
}

pub fn binary_search<T: Ord, I: AsRef<[T]>>(arr: I, element: &T) -> Option<usize> {
    let arr = arr.as_ref();

    if !is_sorted(arr) {
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        match element.cmp(&arr[mid]) {
            Ordering::Less => {
                if mid == 0 {
                    return None;
                } else {
                    high = mid - 1;
                }
            }
            Ordering::Greater => {
                if mid == std::usize::MAX {
                    return None;
                } else {
                    low = mid + 1;
                }
            }
            Ordering::Equal => return Some(mid),
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];

        assert_eq!(linear_search(&v, &1), Some(0));
        assert_eq!(linear_search(&v, &5), Some(4));
        assert_eq!(linear_search(&v, &6), None);
        assert_eq!(linear_search(&v, &0), None);
    }

    #[test]
    fn test_binary_search() {
        let v = (0..1000).collect::<Vec<i32>>();

        assert_eq!(binary_search(&v, &1), Some(1));
        assert_eq!(binary_search(&v, &999), Some(999));
        assert_eq!(binary_search(&v, &1000), None);
        assert_eq!(binary_search(&v, &-1), None);

        let v = vec![5, 4, 3, 2, 1];
        assert_eq!(binary_search(&v, &4), None);
    }
}
