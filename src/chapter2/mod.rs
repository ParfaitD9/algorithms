pub fn find_smallest(array: &[u32]) -> usize {
    assert!(array.len() > 0);
    let mut min_idx: usize = 0;

    for idx in 0..array.len() {
        if array[idx] < array[min_idx] {
            min_idx = idx
        }
    }
    return min_idx;
}

pub fn selection_sort(input: &mut Vec<u32>) -> Vec<u32> {
    let mut output: Vec<u32> = vec![];

    for _ in 0..input.len() {
        let smallest = find_smallest(&input);
        output.push(input[smallest]);
        input.remove(smallest);
    }

    return output;
}
