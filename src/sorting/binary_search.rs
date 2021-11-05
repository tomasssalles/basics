pub fn binary_search(seq: &[i32], target: i32) -> (usize, usize) {
    let mut lb = 0;
    let mut ub = seq.len();
    let mut occs_start = 0;
    let mut occs_end = 0;

    while lb != ub {
        let mid = lb + ((ub-lb) / 2);
        let val = seq[mid];

        if val == target {
            occs_start = mid;
            occs_end = mid + 1;
            break;
        }

        if val < target {
            lb = mid + 1;
        } else {
            ub = mid;
        }
    }

    if lb == ub {
        // target is not in seq
        // Note: Sorted insertion of target would put it at index lb
        // (after moving every element of seq[lb..] one position to the right).
        return (lb, lb);
    }

    while lb != occs_start {
        let mid = lb + ((occs_start-lb) / 2);

        if seq[mid] < target {
            lb = mid + 1;
        } else {
            // seq[mid] == target
            occs_start = mid;
        }
    }

    while occs_end != ub {
        let mid = occs_end + ((ub-occs_end) / 2);

        if seq[mid] == target {
            occs_end = mid + 1;
        } else {
            // seq[mid] > target
            ub = mid;
        }
    }

    return (occs_start, occs_end);
}
