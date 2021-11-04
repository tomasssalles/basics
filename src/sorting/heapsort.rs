use super::{max_heapify, build_max_heap};

pub fn heapsort(seq: &mut [i32]) {
    let n = seq.len();

    if n < 2 {
        return;
    }

    if n > 2 {
        build_max_heap(seq);
        let mut heap_len = n-1;
        seq.swap(0, heap_len);

        while heap_len > 2 {
            max_heapify(&mut seq[..heap_len]);
            heap_len -= 1;
            seq.swap(0, heap_len);
        }
    }

    if seq[0] > seq[1] {
        seq.swap(0, 1);
    }
}
