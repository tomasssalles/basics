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
            let (left_n_buff, right) = (&mut seq[i..(i+(3*k))]).split_at_mut(2*k);
            sandwich_merge_bwd(left_n_buff, k, right);
            i += 4*k;
        }

        if i + (2*k) < n {
            let (left_n_buff, right) = (&mut seq[i..(i+(2*k)+last_chunk)]).split_at_mut(2*k);
            sandwich_merge_bwd(left_n_buff, k, right);
            last_chunk += k;
        } else if i == n {
            last_chunk = 2*k;
        } // else last_chunk stays the same.

        k *= 2;
    }
}

fn sandwich_merge_bwd(seq: &mut [i32], split: usize, right: &mut [i32]) {
    // Merges the presorted sequences left = seq[..split] and right
    // using buffer = seq[split..split+right.len()] as a buffer for temporary values.
    // The resulting sorted sequence is put into seq[..split+right.len()]
    // while the overwritten elements from the buffer are put into right (but
    // most likely not in their original order).
    // Requires that seq.len()-left.len() >= right.len() > 0 and that left.len() > 0.
    // No additional space is allocated, there's no recursion and the runtime
    // is O(left.len()+right.len()).
    let mut l_idx = split - 1;
    let mut l_val = seq[l_idx];
    let mut r_idx = right.len() - 1;
    let mut r_val = right[r_idx];
    let mut write_idx = split + right.len() - 1;

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
            right[r_idx] = seq[write_idx];
            seq[write_idx] = r_val;

            if r_idx == 0 {
                return; // Yes, return!
            }

            write_idx -= 1;
            r_idx -= 1;
            r_val = right[r_idx];
        }
    }

    // Note: r_idx == write_idx.
    slice_swap_safe_left(&mut right[..(r_idx+1)], &mut seq[..(r_idx+1)]);
}

fn mergesort_with_safe_buffer(seq: &mut [i32], buffer: &mut [i32]) {
    // Sorts seq in-place using only buffer as additional space for temporary
    // values AND guarantees that the elements of buffer will be back in it
    // at the end (just scrambled).
    // Runtime is O(n*log(n)) where n = seq.len().
    // buffer.len() must be at least floor(n/2).
    let n = seq.len();

    if n < 2 {
        return;
    }

    let split = n - (n/2);
    mergesort_even_bufferless(seq);
    mergesort_with_safe_buffer(&mut seq[split..], buffer);
    merge_bwd_safe(seq, split, buffer);
}

fn merge_bwd_safe(seq: &mut [i32], split: usize, buffer: &mut [i32]) {
    // Merges the presorted subsequences seq[..split] and seq[split..] in-place using
    // only buffer as additional space for temporary values AND guarantees that the
    // elements of buffer will be back in it at the end (just scrambled).
    // Runtime is O(n) where n = seq.len().
    // buffer.len() must be at least n - split.
    // First, we swap the elements of seq[split..] and buffer. Then we do a sandwich merge.
    slice_swap_safe_left(&mut seq[split..], buffer);
    sandwich_merge_bwd(seq, split, &mut buffer[..seq.len()-split]);
}

fn slice_swap_safe_left(left: &mut [i32], right: &mut [i32]) {
    // Swap the elements of left and right[..left.len()] in O(left.len()) time.
    // Assumes right.len() >= left.len() >= 1. No additional space is allocated.
    // The elements of left will be in the same order after moving to right.
    // The elements of right will be scrambled after moving.
    // This method avoids all (but one) assignments of slice elements to temporary
    // variables, making it more efficient than the usual way of swapping the
    // elements pair by pair.
    let r0 = right[0];
    right[0] = left[0];
    for idx in 1..left.len() {
        left[idx-1] = right[idx];
        right[idx] = left[idx];
    }
    left[left.len()-1] = r0;
}

pub fn mergesort_bufferless(seq: &mut [i32]) {
    // A variant of mergesort (or maybe more like a distant 2nd cousin) which uses
    // only O(log(n)) additional space. The runtime is O(n*log(n)) as usual.
    //
    // There are other variants of mergesort that use O(log(n)) space, but I like this
    // one because the proof of the asymptotic runtime is relatively simple. Still,
    // this may not be the most efficient one. And, of course, heapsort also uses
    // only O(log(n)) space.
    //
    // The detailed algorithm is complicated, but here's a simplified explanation:
    //
    // 1. First we sort the second half of seq using the first half as a buffer.
    // 2. Second, we sort the remaining first half using the 3rd quarter of seq as a
    // buffer (which, yes, scrambles the elements of that 3rd quarter again). Now the
    // 1st half and the 4th quarter are sorted, and additionally all the elements of
    // the 4th quarter are >= than all the elements of the 3rd quarter. This means that
    // the n/4 largest elements of seq must be within the union of the 1st half and the
    // 4th quarter.
    // 3. We merge the two sorted subsequences using the 3rd quarter as a buffer and putting
    // the result in the first 3 quarters of seq, thus leaving the 4th quarter scrambled.
    // Now we know that the 3rd quarter contains the n/4 largest elements of seq.
    // 4. We sort the 4th quarter using the 3rd quarter as a buffer (which scrambles the
    // 3rd quarter again) and then merge the (still sorted) first half with the (now sorted)
    // 4th quarter, putting the result in the first 3 quarters of seq. Now the first 3
    // quarters of seq are sorted, while the 4th quarter is scrambled, and additionally
    // the 4th quarter contains the n/4 largest elements of seq. This means that if we can
    // sort the 4th quarter without touching the rest of the sequence, we are done (i.e.
    // we don't have to merge anything afterwards).
    // 5. To sort the 4th quarter we finally apply recursion.
    //
    // Using recursion only once (instead of every time we need to sort some subsequence)
    // and only on 1/4 of the elements (instead of the usual 1/2) is helpful for two reasons:
    // * First, this algorithm is not very efficient in the sense that some elements get
    // sorted, then scrambled and then sorted again. If we can use a more traditional
    // mergesort for most subsequences (i.e. whenever a buffer is available), that's
    // probably for the best.
    // * Second, it helps in the formal proof (by induction) that the resulting runtime
    // is O(n*log(n)).
    //
    // See? That wasn't complicated at all. Now, these "halfs" and "quarters" are in practice
    // only "close" to n/2 and n/4, but we have to be careful about some size requirements.
    // After the first step, lets assume p elements are scrambled on the left and q elements
    // are sorted on the right. We need q to be even, say equal to 2*r for some r, and we
    // need r >= floor(p/2).
    // Case n % 4 == 0: n == 4*r, p == 2*r, floor(p/2) == r. OK!
    // Case n % 4 == 1: n == (4*r) + 1, p == (2*r) + 1, floor(p/2) == r. OK!
    // Case n % 4 == 2: n == (4*r) - 2, p == (2*r) - 2, floor(p/2) == r - 1. OK!
    // Case n % 4 == 3: n == (4*r) - 1, p == (2*r) - 1, floor(p/2) == r - 1. OK!
    // Ideally, we also want p >= r, as it helps sort the right half in step 1. Requiring
    // n >= 4 is sufficient for this.
    let n = seq.len();

    if n < 4 {
        // Tiny, constant size (1) buffer doesn't count.
        mergesort_with_safe_buffer(seq, &mut [0]);
        return;
    }

    let r = match n % 4 {
        0 => n/4,
        1 => (n-1)/4,
        2 => (n+2)/4,
        3 => (n+1)/4,
        _ => unreachable!(),
    };

    let l_split = n - (2*r);
    let r_split = n - r;

    // Note: l_split/2 <= r == (r_split - l_split) == (n - r_split)
    // and ((n - l_split) / 2) == r <= l_split.

    {
        let (first_half, second_half) = seq.split_at_mut(l_split);
        mergesort_with_safe_buffer(second_half, first_half);
        mergesort_with_safe_buffer(first_half, &mut second_half[..r]);
    }

    let (three_quarters, fourth_quarter) = seq.split_at_mut(r_split);
    sandwich_merge_bwd(three_quarters, l_split, fourth_quarter);
    mergesort_with_safe_buffer(fourth_quarter, &mut three_quarters[l_split..]);
    sandwich_merge_bwd(three_quarters, l_split, fourth_quarter);
    mergesort_bufferless(fourth_quarter);
}
