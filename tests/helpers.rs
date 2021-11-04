pub fn get_all_possible_vectors(len: u32, max_val: i32) -> Vec<Vec<i32>> {
    // Returns all possible vectors of length exactly len with elements
    // among 1, 2, 3, ..., max_val (inclusive). The elements need not be unique.
    if len == 0 {
        return vec![vec![]]
    }

    let mut res: Vec<Vec<i32>> = Vec::with_capacity(i32::pow(max_val, len).try_into().unwrap());

    for vec in get_all_possible_vectors(len-1, max_val) {
		for i in 1..=max_val {
			let mut vec_cp = vec.clone();
			vec_cp.push(i);
			res.push(vec_cp);
		}
	}

    return res;
}
