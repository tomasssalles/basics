pub fn binary_search(seq: &[i32], target: i32) -> (usize, usize) {
    let n = seq.len();

    if n == 1 {
        if seq[0] < target {
            return (1, 1);
        }
        if seq[0] == target {
            return (0, 1);
        }
    }

    return (0, 0);
}
