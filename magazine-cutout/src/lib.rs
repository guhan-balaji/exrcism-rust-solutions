// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{
    collections::HashMap,
    ops::{AddAssign, Sub, SubAssign},
};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map_from_magazine = HashMap::new();
    for word in magazine {
        map_from_magazine.entry(word).or_insert(0).add_assign(1);
    }

    for word in note {
        match map_from_magazine.get_mut(word) {
            Some(count) if *count == 0 => return false,
            Some(count) => count.sub_assign(1),
            None => return false,
        }
    }
    true
}
