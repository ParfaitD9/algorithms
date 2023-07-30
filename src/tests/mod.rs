use crate::chapter1::binary_search;

#[test]
fn found_existing_idx() {
    let guess = binary_search(&[1, 3, 8, 19, 24, 32], 8).unwrap();
    assert_eq!(guess, 2)
}

#[test]
fn found_inexisting_idx() {
    let guess = binary_search(&[1, 3, 8, 19, 24, 32], 9);
    assert_eq!(guess, None)
}
