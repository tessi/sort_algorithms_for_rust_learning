pub fn sort(list: &[u64]) -> &[u64] {
    let mut sorted = list.clone();

    for i in 0..64 {
        sorted = bitsort(sorted, i);
    }
    sorted
}

fn bitsort(list: &[u64], bit: u8) -> &[u64] {
    assert!(bit < 64);
    let mask = 1 << bit;
    let mut counts = [0, 0];

    for element in list.iter() {
        if element & mask == 0 {
            counts[0] = counts[0] + 1;
        } else {
            counts[1] = counts[1] + 1;
        }
    }

    let mut sorted = list.clone();
    let mut zeroes_insert_pos = 0;
    let mut ones_insert_pos = counts[0];
    for element in list.iter() {
        if element & mask == 0 {
            sorted[zeroes_insert_pos] = *element;
            zeroes_insert_pos = zeroes_insert_pos + 1;
        } else {
            sorted[ones_insert_pos] = *element;
            ones_insert_pos = ones_insert_pos + 1;
        }
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_the_empty_array() {
        assert_eq!(sort(&[]), &[]);
    }

    #[test]
    fn it_sorts_one_element_arrays() {
        assert_eq!(sort(&[42]), &[42]);
    }

    #[test]
    fn it_keeps_a_sorted_array_sorted() {
        assert_eq!(sort(&[1, 2, 42]), &[1, 2, 42]);
    }

    #[test]
    fn it_sorts_an_unsorted_array() {
        assert_eq!(sort(&[4, 3, 1, 2]), &[1, 2, 3, 4]);
    }
}
