// https://leetcode.com/problems/group-anagrams/

use std::{collections::HashMap, usize};

fn solution(strs: Vec<&str>) -> Vec<Vec<&str>> {
    let mut anagrams: HashMap<[i32; 26], Vec<&str>> = HashMap::new();

    for s in strs {
        let mut count = [0; 26];

        for c in s.chars() {
            count[(c as usize) - ('a' as usize)] += 1;
        }
        anagrams.entry(count).or_insert(Vec::new()).push(s);
    }
    anagrams.into_values().collect()
}

#[cfg(test)]
mod tests {
    use crate::solutions::s0004_group_anagrams::solution;

    #[test]
    fn should_return_true() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]].sort();

        assert_eq!(solution(strs).sort(), expected);
    }
}
