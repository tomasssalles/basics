use rand::distributions::{Distribution, Uniform};
use rand::{Rng, thread_rng};

use basics::sorting::{bubblesort, quicksort};

trait Sorter {
    fn sort(&self, seq: &mut [i32]);
}

#[derive(Debug)]
struct SortCase<'a> {
    seq: &'a [i32],
    expected: &'a [i32],
    note: &'a str,
}

fn test_sorting_algo(sorter: impl Sorter) {
    let mut cases = vec![
        SortCase{
            seq: &[],
            expected: &[],
            note: "empty",
        },
        SortCase{
            seq: &[42],
            expected: &[42],
            note: "singleton",
        },
        SortCase{
            seq: &[-42, 42],
            expected: &[-42, 42],
            note: "sorted pair with negative",
        },
        SortCase{
            seq: &[42, 42],
            expected: &[42, 42],
            note: "repeating pair",
        },
        SortCase{
            seq: &[0, 1],
            expected: &[0, 1],
            note: "sorted pair with 0",
        },
        SortCase{
            seq: &[1, 3, 5],
            expected: &[1, 3, 5],
            note: "sorted triple",
        },
        SortCase{
            seq: &[-2, 3, 3],
            expected: &[-2, 3, 3],
            note: "sorted triple repeating second",
        },
        SortCase{
            seq: &[1, -3],
            expected: &[-3, 1],
            note: "unsorted pair",
        },
        SortCase{
            seq: &[1, -3, 2],
            expected: &[-3, 1, 2],
            note: "unsorted triple with swap at beginning",
        },
        SortCase{
            seq: &[1, 3, 2],
            expected: &[1, 2, 3],
            note: "unsorted triple with swap at end",
        },
        SortCase{
            seq: &[3, 2, 1],
            expected: &[1, 2, 3],
            note: "unsorted triple with swap of non-adjacent",
        },
        SortCase{
            seq: &[1, 3, 0],
            expected: &[0, 1, 3],
            note: "unsorted triple with 2 swaps forward",
        },
        SortCase{
            seq: &[3, 0, 1],
            expected: &[0, 1, 3],
            note: "unsorted triple with 2 swaps backward",
        },
    ];

    let mut rng = thread_rng();
    let len_range = Uniform::from(0 .. 4000);
    let nums_range = Uniform::from(-1000 .. 1000);
    let mut rand_cases: Vec<(Vec<i32>, Vec<i32>, String)> = Vec::with_capacity(10);

    for _ in 0 .. rand_cases.capacity() {
        let len = len_range.sample(&mut rng);
        let vec: Vec<i32> = (&mut rng).sample_iter(nums_range).take(len).collect();
        let mut vec_cp = vec.clone();
        vec_cp.sort();
        let note = format!("random with len={}", len);

        rand_cases.push((vec, vec_cp, note));
    }

    for (v, v_sorted, note) in &rand_cases {
        cases.push(SortCase {seq: &v[..], expected: &v_sorted[..], note: &note});
    }

    for case in cases {
        let mut seq_cp = case.seq.to_vec();
        sorter.sort(seq_cp.as_mut_slice());
        assert_eq!(case.expected, seq_cp, "note: {}, original seq: {:?}", case.note, case.seq);
    }
}

struct Bubblesorter;

impl Sorter for Bubblesorter {
    fn sort(&self, seq: &mut [i32]) {
        return bubblesort(seq);
    }
}

#[test]
fn test_bubblesort() {
    test_sorting_algo(Bubblesorter);
}

struct Quicksorter;

impl Sorter for Quicksorter {
    fn sort(&self, seq: &mut [i32]) {
        return quicksort(seq);
    }
}

#[test]
fn test_quicksort() {
    test_sorting_algo(Quicksorter);
}
