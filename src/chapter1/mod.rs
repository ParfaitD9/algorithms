/// Found index of element in an array
pub fn binary_search(array: &[u32], element: u32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let pos = ((high + low) / 2) as usize;
        if element == array[pos] {
            return Some(pos);
        } else if element > array[pos] {
            low = pos + 1
        } else {
            high = pos - 1
        }
    }
    return None;
}
