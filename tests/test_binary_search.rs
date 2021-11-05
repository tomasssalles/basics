use basics::sorting::binary_search;

#[test]
fn test_binary_search_empty() {
    assert_eq!(binary_search(&[], 42), (0, 0));
}

#[test]
fn test_binary_search_singleton_missing() {
    assert_eq!(binary_search(&[99], 42), (0, 0));
    assert_eq!(binary_search(&[7], 42), (1, 1));
}

#[test]
fn test_binary_search_singleton_present() {
    assert_eq!(binary_search(&[42], 42), (0, 1));
}

#[test]
fn test_binary_search_pair_missing() {
    assert_eq!(binary_search(&[99, 101], 42), (0, 0));
    assert_eq!(binary_search(&[7, 101], 42), (1, 1));
    assert_eq!(binary_search(&[7, 23], 42), (2, 2));
}

#[test]
fn test_binary_search_pair_once() {
    assert_eq!(binary_search(&[42, 99], 42), (0, 1));
    assert_eq!(binary_search(&[7, 42], 42), (1, 2));
}

#[test]
fn test_binary_search_large_missing() {
    assert_eq!(binary_search(&[76, 76, 77, 78, 99, 101, 101], 42), (0, 0));
    assert_eq!(binary_search(&[-7, 0, 0, 1, 101], 42), (4, 4));
    assert_eq!(binary_search(&[7, 8, 13, 14, 15, 23, 23, 23], 42), (8, 8));
}

#[test]
fn test_binary_search_large_once() {
    assert_eq!(binary_search(&[42, 50, 51, 51, 60, 99], 42), (0, 1));
    assert_eq!(binary_search(&[7, 9, 42, 43, 88, 99], 42), (2, 3));
    assert_eq!(binary_search(&[-3, -2, -1, 0, 1, 42], 42), (5, 6));
}

// #[test]
// fn test_binary_search_pair_repeated() {
//     assert_eq!(binary_search(&[42, 42], 42), (0, 2));
// }

// #[test]
// fn test_binary_search_large_repeated() {
//     assert_eq!(binary_search(&[42, 42, 42, 50, 51, 51, 60, 99], 42), (0, 3));
//     assert_eq!(binary_search(&[7, 9, 42, 42, 42, 42, 43, 88, 99], 42), (2, 6));
//     assert_eq!(binary_search(&[-3, -2, -1, 0, 1, 42, 42], 42), (5, 7));
//     assert_eq!(binary_search(&[42, 42, 42, 42, 42, 42, 42], 42), (0, 7));
// }
