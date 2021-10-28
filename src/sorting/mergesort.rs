pub fn mergesort(seq: &mut [i32]) {
    let mut buffer: Vec<i32> = vec![0; seq.len()];
    mergesort_with_buffer(seq, &mut buffer[..]);
}

fn mergesort_with_buffer(seq: &mut [i32], buffer: &mut [i32]) {
    if seq.len() < 2 {
        return;
    }

    let split = seq.len() / 2;
    mergesort_with_buffer(&mut seq[..split], buffer);
    mergesort_with_buffer(&mut seq[split..], buffer);
    merge_fwd(seq, split, &mut buffer[..split]);
}

fn merge_fwd(seq: &mut [i32], split: usize, buffer: &mut [i32]) {
    buffer.copy_from_slice(&seq[..split]);

    let mut write_idx = 0;
    let mut l_idx = 0;
    let mut r_idx = split;

    while l_idx < split && r_idx < seq.len() {
        let l_val = buffer[l_idx];
        let r_val = seq[r_idx];

        if l_val <= r_val {
            seq[write_idx] = l_val;
            l_idx += 1;
        } else {
            seq[write_idx] = r_val;
            r_idx += 1;
        }

        write_idx += 1;
    }

    while l_idx < split {
        seq[write_idx] = buffer[l_idx];
        l_idx += 1;
        write_idx += 1;
    }
}
