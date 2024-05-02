// https://leetcode.com/problems/two-sum/description/

#![allow(unused)]

use std::collections::HashMap;

fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let index = index as i32;
        let needed = target - num;

        if map.contains_key(&needed) {
            return vec![map[&needed], index];
        }

        map.insert(num, index);
    }

    Vec::new()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_0_and_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];

        assert_eq!(solution(nums, target), expected);
    }

    #[test]
    fn it_should_return_empty() {
        let nums = vec![2, 7, 11, 15];
        let target = 25;
        let expected = vec![];

        assert_eq!(solution(nums, target), expected);
    }
}
