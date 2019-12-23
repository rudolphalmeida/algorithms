pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            if arr[j - 1] <= arr[j] {
                break;
            }
            arr.swap(j, j - 1);
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let min_index = (i..arr.len())
            .zip(&arr[i..])
            .min_by(|(_, x), (_, y)| x.cmp(&y))
            .unwrap()
            .0;
        arr.swap(i, min_index);
    }
}

pub fn merge_sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len >= 2 {
        // Single elements are trivially sorted
        let (left, right) = arr.split_at_mut(len / 2);
        merge_sort(left);
        merge_sort(right);
        merge(left, right);
    }
}

fn merge<T: Ord + Clone + Copy>(left: &mut [T], right: &mut [T]) {
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

        if right_iter.peek().unwrap() < left_iter.peek().unwrap() {
            *elem = *right_iter.next().unwrap();
        } else {
            *elem = *left_iter.next().unwrap();
        }
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len >= 2 {
        let pivot = partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    arr.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if arr[i] < arr[last_index] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }

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
        let mut v = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        // Test stable sort
        let mut v = sample_data();
        bubble_sort(&mut v);
        assert_eq!(v, sorted_sample_data());
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        // Test stable sort
        let mut v = sample_data();
        insertion_sort(&mut v);
        assert_eq!(v, sorted_sample_data());
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
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        // Test stable sort
        let mut v = sample_data();
        merge_sort(&mut v);
        assert_eq!(v, sorted_sample_data());
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
