use super::max_heapify;

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

fn _max_heapify(seq: &mut [i32], start: usize) {
    let n = seq.len();
    let root = seq[start]; // root of the subtree
    let mut parent_idx = start;

    loop{
        let l_idx = (2*parent_idx)+1;

        if l_idx >= n {
            break;
        }

        let r_idx = l_idx+1;
        let max_child_idx = if (r_idx >= n) || (seq[l_idx] >= seq[r_idx]) {l_idx} else {r_idx};

        if seq[max_child_idx] > root {
            seq[parent_idx] = seq[max_child_idx];
            parent_idx = max_child_idx;
        } else {
            break;
        }
    }

    if parent_idx != start {
        seq[parent_idx] = root;
    }
}

fn build_max_heap(seq: &mut [i32]) {
    // max_heapify does nothing if (2*start)+1 >= seq.len(), i.e.
    // if start >= (seq.len()-1)/2 (fp division). In particular,
    // it does nothing if start >= floor(seq.len()/2).
    for start in (0..(seq.len()/2)).rev() {
        _max_heapify(seq, start);
    }
}
