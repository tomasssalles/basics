use rand::distributions::{Distribution, Uniform};
use rand::{Rng, thread_rng};

use basics::sorting::bubblesort;

fn check_sorted(arr: &[i32]) {
    let n = arr.len();
    if n < 2 { return; }

    for i in 0 .. n - 1 {
        assert!(
            arr[i] <= arr[i+1],
            "i={}, arr[i]={}, arr[i+1]={}, n={}", i, arr[i], arr[i+1], n
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        check_sorted(&[]);
    }

    #[test]
    fn singleton() {
        check_sorted(&[-99]);
    }

    #[test]
    fn sorted() {
        check_sorted(&[1, 2]);
        check_sorted(&[45, 500, 501, 1001]);
        check_sorted(&[5, 5, 6, 6, 9, 9]);
        check_sorted(&[0, 0, 0, 0, 0]);
    }

    #[test]
    #[should_panic(expected = "i=0, arr[i]=4, arr[i+1]=3, n=2")]
    fn unsorted_small() {
        check_sorted(&[4, 3]);
    }

    #[test]
    #[should_panic(expected = "i=2, arr[i]=4, arr[i+1]=3, n=7")]
    fn unsorted_middle() {
        check_sorted(&[-1, 3, 4, 3, 5, 6, 7]);
    }

    #[test]
    #[should_panic(expected = "i=3, arr[i]=4, arr[i+1]=3, n=5")]
    fn unsorted_end() {
        check_sorted(&[-1, 0, 2, 4, 3]);
    }
}

#[test]
fn empty() {
    let mut arr = [];
    bubblesort(&mut arr);
}

#[test]
fn singleton() {
    let mut arr = [42];
    bubblesort(&mut arr);
    assert_eq!([42], arr);
}

#[test]
fn sorted_small() {
    let mut arr1 = [-42, 42];
    bubblesort(&mut arr1);
    assert_eq!([-42, 42], arr1);

    let mut arr2 = [42, 42];
    bubblesort(&mut arr2);
    assert_eq!([42, 42], arr2);

    let mut arr3 = [0, 1];
    bubblesort(&mut arr3);
    assert_eq!([0, 1], arr3);

    let mut arr4 = [1, 3, 5];
    bubblesort(&mut arr4);
    assert_eq!([1, 3, 5], arr4);

    let mut arr5 = [-2, 3, 3];
    bubblesort(&mut arr5);
    assert_eq!([-2, 3, 3], arr5);
}

#[test]
fn unsorted_small() {
    let mut arr1 = [1, -3];
    bubblesort(&mut arr1);
    assert_eq!([-3, 1], arr1);

    let mut arr2 = [1, -3, 2];
    bubblesort(&mut arr2);
    assert_eq!([-3, 1, 2], arr2);

    let mut arr3 = [1, 3, 2];
    bubblesort(&mut arr3);
    assert_eq!([1, 2, 3], arr3);

    let mut arr4 = [1, 3, 0];
    bubblesort(&mut arr4);
    assert_eq!([0, 1, 3], arr4);
}

#[test]
fn unsorted_large() {
    let mut rng = thread_rng();

    let len_range = Uniform::from(0 .. 4000);
    let nums_range = Uniform::from(-1000 .. 1000);

    for _ in 0..10 {
        let len = len_range.sample(&mut rng);
        let mut vec: Vec<i32> = (&mut rng)
            .sample_iter(nums_range).take(len).collect();
        let mut expected = vec.clone();
        expected.sort();

        bubblesort(&mut vec);
        assert_eq!(expected, vec);
    }
}
