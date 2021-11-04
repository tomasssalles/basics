pub fn max_heapify(seq: &mut [i32]) {
    if seq.len() >= 2 && seq[0] < seq[1] {
        seq.swap(0, 1);
    }
}
