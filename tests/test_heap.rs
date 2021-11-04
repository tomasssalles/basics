use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use basics::sorting::{max_heapify};

fn check_have_same_elements(l_seq: &[i32], r_seq: &[i32]) {
    let mut l_seq_cp = l_seq.to_vec();
    let mut r_seq_cp = r_seq.to_vec();
    l_seq_cp.sort();
    r_seq_cp.sort();
    assert_eq!(
        l_seq_cp,
        r_seq_cp,
        "Sequences differ after sorting! Original:\nl_seq={:?},\nr_seq={:?}",
        l_seq,
        r_seq,
    );
}

fn check_is_max_heap(seq: &[i32]) {
    for i in 0..seq.len() { // Will return earlier
        let l_child = (2*i)+1;

        if l_child >= seq.len() {
            return;
        }

        assert!(seq[i] >= seq[l_child], "i={}, l_child={}, seq={:?}", i, l_child, seq);

        let r_child = l_child+1;

        if r_child >= seq.len() {
            return;
        }

        assert!(seq[i] >= seq[r_child], "i={}, r_child={}, seq={:?}", i, r_child, seq);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_max_heap_empty() {
        check_is_max_heap(&[]);
    }

    #[test]
    fn test_is_max_heap_singleton() {
        check_is_max_heap(&[42]);
    }

    #[test]
    fn test_is_max_heap_pair() {
        check_is_max_heap(&[42, 41]);
        check_is_max_heap(&[42, 42]);
    }

    #[test]
    fn test_is_max_heap_triple() {
        check_is_max_heap(&[42, 13, 14]);
        check_is_max_heap(&[42, 13, 13]);
        check_is_max_heap(&[42, 15, 14]);
        check_is_max_heap(&[42, 42, 14]);
        check_is_max_heap(&[42, 13, 42]);
        check_is_max_heap(&[42, 42, 42]);
    }

    #[test]
    fn test_is_max_heap_large() {
        check_is_max_heap(&[5, 4, 3, 2, 1, 0]);
        check_is_max_heap(&[5, 4, 4, 3, -1, 0, 4]);
        check_is_max_heap(&[5, 4, 3, 3, 2, 2, 1, 0, -1, -2]);
        check_is_max_heap(&[5, 0, 4, -2, -1, 1, 3, -5]);
    }
}

#[test]
fn test_max_heapify_empty() {
    let mut seq: [i32; 0] = [];
    max_heapify(&mut seq);
}

#[test]
fn test_max_heapify_singleton() {
    let mut seq = [42];
    max_heapify(&mut seq);
    assert_eq!([42], seq);
}

#[test]
fn test_max_heapify_already_heap() {
    let seqs = [
        vec![42, 7],
        vec![42, 7, 8],
        vec![5, 4, 4],
        vec![6, 6, 6, 6],
        vec![9, 9, 3, 1, 2],
        vec![9, 9, 3, 1, 2, 1, 2, 1],
        vec![42, 3, 31, 2, 1, 25, 25, 1, 2, -1, -5, 23]
    ];

    for seq in seqs {
        let mut seq_cp = seq.clone();
        max_heapify(&mut seq_cp);
        assert_eq!(seq, seq_cp);
    }
}

#[test]
fn test_max_heapify_pair() {
    let mut seq = [42, 43];
    max_heapify(&mut seq);
    assert_eq!([43, 42], seq);
}

#[test]
fn test_max_heapify_depth_2() {
    let seqs = [
        vec![42, 13, 43],
        vec![42, 43, 13],
        vec![42, 42, 43],
        vec![42, 43, 42],
        vec![42, 43, 43],
        vec![42, 88, 99],
        vec![42, 99, 88],
    ];

    for seq in seqs {
        let mut seq_cp = seq.clone();
        max_heapify(&mut seq_cp);
        check_have_same_elements(&seq, &seq_cp);
        check_is_max_heap(&seq_cp);
    }
}

#[test]
fn test_max_heapify_depth_3() {
    let seqs = [
        // Move one level down
        vec![42, 55, 11, 14],
        vec![42, 55, 11, 14, 17],
        vec![42, 55, 11, 14, 17,  5],
        vec![42, 55, 11, 14, 17,  5,  3],
        vec![42, 11, 55,  5],
        vec![42, 11, 55,  5,  3],
        vec![42, 11, 55,  5,  3, 14],
        vec![42, 11, 55,  5,  3, 14, 17],
        // Move two levels down
        vec![10, 55, 11, 14],
        vec![10, 55, 11, 14, 17],
        vec![10, 55, 11, 17, 14],
        vec![ 3, 55, 11, 14, 17,  5],
        vec![ 4, 55, 11, 14,  7,  5,  3],
        vec![ 9, 11, 55,  5,  3, 14],
        vec![16, 11, 55,  5,  3, 14, 17],
    ];

    for seq in seqs {
        let mut seq_cp = seq.clone();
        max_heapify(&mut seq_cp);
        check_have_same_elements(&seq, &seq_cp);
        check_is_max_heap(&seq_cp);
    }
}

fn get_random_max_heap() -> Vec<i32> { // Minimum length is 3
    let mut rng = thread_rng();
    let deltas_range = Uniform::from(0 .. 1000);
    let len = Uniform::from(3 .. 4000).sample(&mut rng);
    let mut vec = Vec::<i32>::with_capacity(len);

    vec.push(Uniform::from(5000..20000).sample(&mut rng));

    for child in 1..len {
        let parent = if child % 2 == 1 {
            (child-1)/2
        } else {
            (child-2)/2
        };

        vec.push(vec[parent] - deltas_range.sample(&mut rng));
    }

    return vec;
}

#[test]
fn test_max_heapify_random_already_heap() {
    for _ in 0..10 {
        let vec = get_random_max_heap();
        let mut vec_cp = vec.clone();
        max_heapify(&mut vec_cp);
        assert_eq!(vec, vec_cp);
    }
}

#[test]
fn test_max_heapify_random() {
    let mut rng = thread_rng();

    for _ in 0..10 {
        let mut vec = get_random_max_heap();
        vec[0] = Uniform::from(-2000..std::cmp::max(vec[1], vec[2])).sample(&mut rng);

        let mut vec_cp = vec.clone();
        max_heapify(&mut vec_cp);
        check_have_same_elements(&vec, &vec_cp);
        check_is_max_heap(&vec_cp);
    }
}
