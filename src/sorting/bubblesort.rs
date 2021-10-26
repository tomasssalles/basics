pub fn bubblesort(arr: &mut [i32]) {
    for j in (1 .. arr.len()).rev() {
        let mut done = true;

        for i in 0 .. j {
            if arr[i] > arr[i+1] {
                let l_val = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = l_val;
                done = false;
            }
        }

        if done { return; }
    }
}
