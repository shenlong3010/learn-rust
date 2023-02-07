use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::algo_search::linear_search;

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &vec!["a", "b", "c", "d", "e", "f"]);
        assert_eq!(index, Some(0));
    }
}
