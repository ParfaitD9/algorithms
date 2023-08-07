#[cfg(test)]
mod chapter1 {
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
}

mod chapter2 {
    use crate::chapter2;

    #[test]
    fn return_smallest() {
        let data = [8, 2, 5, 4, 2, 5, 1];
        assert_eq!(chapter2::find_smallest(&data), 6)
    }

    #[test]
    fn sort() {
        let mut input = Vec::from([8, 2, 5, 4, 2, 5, 1]);
        let expected = Vec::from([1, 2, 2, 4, 5, 5, 8]);

        let output = chapter2::selection_sort(&mut input);
        println!("{:?}", output);
        assert_eq!(output, expected);
    }
}
