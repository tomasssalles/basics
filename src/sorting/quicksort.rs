pub fn quicksort(seq: &mut [i32]) {
    if seq.len() < 2 {
        return;
    }

    let mut pivot_idx = 0;
    let pivot_val = seq[0];

    for idx in 1 .. seq.len() {
        let val = seq[idx];

        if val < pivot_val {
            seq[pivot_idx] = val;

            pivot_idx += 1;

            if idx > pivot_idx {
                seq[idx] = seq[pivot_idx];
            }
        }
    }

    seq[pivot_idx] = pivot_val;
    quicksort(&mut seq[..pivot_idx]);
    quicksort(&mut seq[(pivot_idx+1)..]);
}
