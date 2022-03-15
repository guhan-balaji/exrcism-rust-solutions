use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut letter_count = HashMap::new();
    for c in word.to_lowercase().chars() {
        letter_count.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }

    possible_anagrams
        .iter()
        .filter(|&w| match w.to_lowercase() {
            w if w == word.to_lowercase() => false,
            w => {
                let mut letter_count = letter_count.clone();
                for c in w.chars() {
                    match letter_count.get_mut(&c) {
                        Some(x) if *x == 0 => return false,
                        Some(x) => *x -= 1,
                        None => return false,
                    }
                }
                letter_count.into_values().sum::<i32>() == 0
            }
        })
        .map(|&x| x)
        .collect::<HashSet<&'a str>>()
}
