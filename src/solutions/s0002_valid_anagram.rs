// https://leetcode.com/problems/valid-anagram/description/

#![allow(unused)]

use std::collections::HashMap;

fn solution(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|v| v == 0)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_return_true() {
        let s = String::from("anagram");
        let t = String::from("nagaram");

        assert_eq!(solution(s, t), true);
    }

    #[test]
    fn it_should_return_false() {
        let s = String::from("rat");
        let t = String::from("car");

        assert_eq!(solution(s, t), false);
    }
}
