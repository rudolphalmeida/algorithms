pub fn bubble_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if cmp(&arr[i + 1], &arr[i]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn insertion_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            if cmp(&arr[j - 1], &arr[j]) {
                break;
            }
            arr.swap(j, j - 1);
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    // TODO: Take a comparison function
    for i in 0..arr.len() - 1 {
        let min_index = (i..arr.len())
            .zip(&arr[i..])
            .min_by(|(_, x), (_, y)| x.cmp(&y))
            .unwrap()
            .0;
        arr.swap(i, min_index);
    }
}

pub fn merge_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord + Copy + Clone,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len >= 2 {
        // Single elements are trivially sorted
        let (left, right) = arr.split_at_mut(len / 2);
        merge_sort(left, cmp);
        merge_sort(right, cmp);
        merge(left, right, cmp);
    }
}

fn merge<T, F>(left: &mut [T], right: &mut [T], cmp: &F)
where
    T: Ord + Clone + Copy,
    F: Fn(&T, &T) -> bool,
{
    // Copy elements into vectors and use peekable iterators
    // This avoids appending a sentinel `infinity` value at the end
    // to check if a vector has been exhausted
    let left_vec = left.to_vec();
    let mut left_iter = left_vec.iter().peekable();
    let right_vec = right.to_vec();
    let mut right_iter = right_vec.iter().peekable();

    for elem in left.iter_mut().chain(right.iter_mut()) {
        // left has exhausted
        if left_iter.peek().is_none() {
            *elem = *right_iter.next().unwrap();
            continue;
        }

        // right has exhausted
        if right_iter.peek().is_none() {
            *elem = *left_iter.next().unwrap();
            continue;
        }

        if cmp(right_iter.peek().unwrap(), left_iter.peek().unwrap()) {
            *elem = *right_iter.next().unwrap();
        } else {
            *elem = *left_iter.next().unwrap();
        }
    }
}

pub fn quick_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len >= 2 {
        let pivot = partition(arr, cmp);
        quick_sort(&mut arr[..pivot], cmp);
        quick_sort(&mut arr[pivot + 1..], cmp);
    }
}

fn partition<T, F>(arr: &mut [T], cmp: &F) -> usize
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    // Move pivot element to end
    arr.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if cmp(&arr[i], &arr[last_index]) {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }

    // Move pivot element to its position in the sorted array
    arr.swap(store_index, last_index);
    store_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
    struct Data {
        pub key: i32,
        pub satellite_data: i32,
    }

    fn sample_data() -> Vec<Data> {
        vec![
            Data {
                key: 1,
                satellite_data: 1,
            },
            Data {
                key: 2,
                satellite_data: 2,
            },
            Data {
                key: 3,
                satellite_data: 4,
            },
            Data {
                key: 2,
                satellite_data: 3,
            },
            Data {
                key: 3,
                satellite_data: 5,
            },
        ]
    }

    fn sorted_sample_data() -> Vec<Data> {
        vec![
            Data {
                key: 1,
                satellite_data: 1,
            },
            Data {
                key: 2,
                satellite_data: 2,
            },
            Data {
                key: 2,
                satellite_data: 3,
            },
            Data {
                key: 3,
                satellite_data: 4,
            },
            Data {
                key: 3,
                satellite_data: 5,
            },
        ]
    }

    #[test]
    fn test_bubble_sort() {
        // Sort ascending
        let mut v = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        // Sort ascending
        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        bubble_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        // Sort descending
        let mut v = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut v, &|x, y| x > y);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        // Test stable sort
        let mut v = sample_data();
        bubble_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, sorted_sample_data());

        let mut v = vec!["abcde", "abcd", "abc", "ab", "a"];
        bubble_sort(&mut v, &|x, y| x.len() < y.len());
        assert_eq!(v, vec!["a", "ab", "abc", "abcd", "abcde"]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        insertion_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut v, &|x, y| x > y);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        // Test stable sort
        let mut v = sample_data();
        insertion_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, sorted_sample_data());

        let mut v = vec!["abcde", "abcd", "abc", "ab", "a"];
        insertion_sort(&mut v, &|x, y| x.len() < y.len());
        assert_eq!(v, vec!["a", "ab", "abc", "abcd", "abcde"]);
    }

    #[test]
    fn test_selection_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        merge_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        merge_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        merge_sort(&mut v, &|x, y| x > y);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        // Test stable sort
        let mut v = sample_data();
        merge_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, sorted_sample_data());

        let mut v = vec!["abcde", "abcd", "abc", "ab", "a"];
        merge_sort(&mut v, &|x, y| x.len() < y.len());
        assert_eq!(v, vec!["a", "ab", "abc", "abcd", "abcde"]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        quick_sort(&mut v, &|x, y| x > y);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        // Test stable sort
        let mut v = sample_data();
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, sorted_sample_data());

        let mut v = vec!["abcde", "abcd", "abc", "ab", "a"];
        quick_sort(&mut v, &|x, y| x.len() < y.len());
        assert_eq!(v, vec!["a", "ab", "abc", "abcd", "abcde"]);
    }
}
