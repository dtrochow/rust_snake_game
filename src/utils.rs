use std::collections::HashSet;

pub fn has_duplicate_coordinates(list: &Vec<(i32, i32)>) -> bool {
    let mut seen_coordinates: HashSet<&(i32, i32)> = HashSet::new();

    for coordinate in list.iter() {
        if !seen_coordinates.insert(coordinate) {
            return true;
        }
    }

    false
}
