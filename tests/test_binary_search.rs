use basics::sorting::binary_search;

#[test]
fn test_binary_search_empty() {
    let res = binary_search(&[], 42);
    assert_eq!(res, (0, 0));
}

#[test]
fn test_binary_search_singleton_missing() {
    let res1 = binary_search(&[99], 42);
    assert_eq!(res1, (0, 0));

    let res2 = binary_search(&[7], 42);
    assert_eq!(res2, (1, 1));
}

#[test]
fn test_binary_search_singleton_present() {
    let res = binary_search(&[42], 42);
    assert_eq!(res, (0, 1));
}
