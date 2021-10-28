pub fn mergesort(seq: &mut [i32]) {
    if seq.len() < 2 {
        return;
    }

    let split = seq.len() / 2;

    mergesort(&mut seq[..split]);
    mergesort(&mut seq[split..]);
    merge(seq, split);
}

fn merge(seq: &mut [i32], split: usize) {
    let buffer = seq.to_vec();
    let mut write_idx = 0;
    let mut left_idx = 0;
    let mut right_idx = split;

    while left_idx < split || right_idx < seq.len() {
        if left_idx < split && right_idx < seq.len() {
            if buffer[left_idx] <= buffer[right_idx] {
                seq[write_idx] = buffer[left_idx];
                left_idx += 1;
                write_idx += 1;
            } else {
                seq[write_idx] = buffer[right_idx];
                right_idx += 1;
                write_idx += 1;
            }
        } else if left_idx < split {
            seq[write_idx] = buffer[left_idx];
            left_idx += 1;
            write_idx += 1;
        } else {
            seq[write_idx] = buffer[right_idx];
            right_idx += 1;
            write_idx += 1;
        }
    }
}
