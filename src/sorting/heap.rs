pub fn max_heapify(seq: &mut [i32]) {
    let n = seq.len();

    if n < 2 {
        return;
    }

    let start = 0;
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
