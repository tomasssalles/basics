pub fn bubblesort(seq: &mut [i32]) {
    for j in (1 .. seq.len()).rev() {
        let mut done = true;

        for i in 0 .. j {
            if seq[i] > seq[i+1] {
                let l_val = seq[i];
                seq[i] = seq[i+1];
                seq[i+1] = l_val;
                done = false;
            }
        }

        if done { return; }
    }
}
