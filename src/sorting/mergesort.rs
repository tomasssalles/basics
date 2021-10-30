pub fn mergesort(seq: &mut [i32]) {
    let mut buffer: Vec<i32> = vec![0; seq.len() / 2];
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

fn mergesort_even_bufferless(seq: &mut [i32]) {
    // Sorts the elements of seq that have even indices, putting them in the
    // left "half" (n/2 rounded up) of seq and putting all other elements in the
    // right "half" (n/2 rounded down).
    // The elements that originally had odd indices do not get sorted and also
    // do not keep their original order.
    // No additional space is allocated, there's no recursion and the runtime
    // is O(n*log(n)).
    let n = seq.len();
    let mut k = 1;
    let mut last_chunk = 1;

    while 2*k < n {
        let mut i = 0;

        while i + (4*k) <= n {
            sandwich_merge_bwd(&mut seq[i..(i+(3*k))], k, 2*k);
            i += 4*k;
        }

        if i + (2*k) < n {
            sandwich_merge_bwd(&mut seq[i..(i+(2*k)+last_chunk)], k, 2*k);
            last_chunk += k;
        } else if i == n {
            last_chunk = 2*k;
        } // else last_chunk stays the same.

        k *= 2;
    }
}

fn sandwich_merge_bwd(seq: &mut [i32], l_split: usize, r_split: usize) {
    // Merges the presorted sequences left = seq[..l_split] and right = seq[r_split..]
    // using buffer = seq[l_split..r_split] as a buffer for temporary values.
    // The resulting sorted sequence is put into seq[..left.len()+right.len()]
    // while the original elements from the buffer are put into
    // seq[left.len()+right.len()..] (but most likely not in their original order).
    // Requires that buffer.len() >= right.len() > 0 and that left.len() > 0.
    // No additional space is allocated, there's no recursion and the runtime
    // is O(left.len()+right.len()).
    let mut l_idx = l_split - 1;
    let mut l_val = seq[l_idx];
    let mut r_idx = seq.len() - 1;
    let mut r_val = seq[r_idx];
    let mut write_idx = l_split + (seq.len()-r_split) - 1;

    loop {
        if l_val >= r_val {
            seq[l_idx] = seq[write_idx];
            seq[write_idx] = l_val;
            write_idx -= 1;

            if l_idx == 0 {
                break; // Just break. There's still work to be done.
            }

            l_idx -= 1;
            l_val = seq[l_idx];
        } else {
            seq[r_idx] = seq[write_idx];
            seq[write_idx] = r_val;

            if r_idx == r_split {
                return; // Yes, return!
            }

            write_idx -= 1;
            r_idx -= 1;
            r_val = seq[r_idx];
        }
    }

    loop {
        seq[r_idx] = seq[write_idx];
        seq[write_idx] = r_val;

        if r_idx == r_split {
            return;
        }

        write_idx -= 1;
        r_idx -= 1;
        r_val = seq[r_idx];
    }
}

fn mergesort_with_buffer_even_first(seq: &mut [i32], buffer: &mut [i32]) {
    let n = seq.len();

    if n < 2 {
        return;
    }

    let split = n - (n/2);
    mergesort_even_bufferless(seq);
    mergesort_with_buffer_even_first(&mut seq[split..], buffer);
    merge_fwd(seq, split, &mut buffer[..split]);
}

pub fn mergesort2(seq: &mut [i32]) {
    let n = seq.len();
    let mut buffer: Vec<i32> = vec![0; n - (n/2)];
    mergesort_with_buffer_even_first(seq, &mut buffer[..]);
}

pub fn mergesort_bufferless(seq: &mut [i32]) {
    let n = seq.len();

    if n < 4 {
        // Tiny, constant size (1) buffer doesn't count.
        mergesort_with_buffer(seq, &mut [0]);
        return;
    }

    // TODO
}
