// https://leetcode.com/problems/contains-duplicate/description/

#![allow(unused)]

use std::collections::HashSet;

fn solution(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_true() {
        let nums = vec![1, 4, 5, 5, 2, 1];
        assert_eq!(solution(nums), true);
    }
}
