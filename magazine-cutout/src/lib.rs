// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;


pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // let mut map: HashMap<&str, i8> = HashMap::new();
    // for item in magazine {
    //     let mut v = 1 as i8;
    //     if map.contains_key(item) {
    //         v = map.get(item).unwrap() + 1;
    //     }
    //     map.insert(item, v);
    // }
    // for item in note {
    //     if !map.contains_key(item) {
    //         return false;
    //     }
    //     let v = map.get(item).unwrap() - 1;
    //     if v < 0 {
    //         return false;
    //     }
    //     map.insert(item, v);
    // }
    // return true;
    let magazine_map = magazine.iter()
        .fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        });

    let note_map = note.iter()
        .fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        });

    note_map.iter()
        .all(|(&key, &count)| {
            magazine_map.get(&key).unwrap_or(&0) >= &count
        })
}
