pub fn binary_search(seq: &[i32], target: i32) -> (usize, usize) {
    let mut lb = 0;
    let mut ub = seq.len();

    while lb != ub {
        let mid = lb + ((ub-lb) / 2);
        let val = seq[mid];

        if val == target {
            return (mid, mid+1);
        }

        if val < target {
            lb = mid+1;
        } else {
            ub = mid;
        }
    }

    return (lb, ub);
}
