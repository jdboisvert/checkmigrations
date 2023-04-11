use std::collections::HashSet;

pub fn has_duplicates<T: std::hash::Hash + std::cmp::Eq>(vec: &[T]) -> bool {
    let mut seen = HashSet::new();

    for item in vec {
        if !seen.insert(item) {
            return true;
        }
    }

    false
}

#[cfg(test)]
#[path = "./lib_tests.rs"]
mod lib_tests;
