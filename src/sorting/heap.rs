pub fn max_heapify(seq: &mut [i32]) {
    if seq.len() < 2 {
        return;
    }

    _max_heapify(seq, 0);
}

fn _max_heapify(seq: &mut [i32], start: usize) {
    // Requires: start must be a valid index for seq!
    let n = seq.len();
    let root = seq[start];
    let mut idx = start;

    loop {
        let l_child = (2*idx)+1;

        if l_child >= n {
            break;
        }

        let r_child = l_child+1;

        let max_child = if (r_child >= n) || (seq[l_child] >= seq[r_child]) {
            l_child
        } else {
            r_child
        };

        if root >= seq[max_child] {
            break;
        }

        seq[idx] = seq[max_child];
        idx = max_child;
    }

    if idx != start {
        seq[idx] = root;
    }
}

pub fn build_max_heap(seq: &mut [i32]) {
    // _max_heapify does nothing if (2*start)+1 >= seq.len(), i.e.
    // if start >= (seq.len()-1)/2 (fp division). In particular,
    // it does nothing if start >= floor(seq.len()/2).
    for start in (0..(seq.len()/2)).rev() {
        _max_heapify(seq, start);
    }
}
