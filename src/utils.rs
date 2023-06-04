use std::collections::{LinkedList, HashSet};

pub fn has_duplicate_coordinates(list: &LinkedList<(i32, i32)>) -> bool {
    let mut seen_coordinates: HashSet<&(i32, i32)> = HashSet::new();

    for coordinate in list.iter() {
        if !seen_coordinates.insert(coordinate) {
            return true;
        }
    }

    false
}

pub fn penultimate_element<T>(list: &LinkedList<T>) -> Option<&T> {
    let mut iter = list.iter();
    let mut prev = None;
    let mut current = None;

    while let Some(item) = iter.next() {
        prev = current;
        current = Some(item);
    }

    prev
}
