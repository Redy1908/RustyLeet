// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/

use std::collections::HashMap;

fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut p1 = 0;
    let mut p2 = nums.len() - 1;

    while p1 < p2 {
        let sum = nums[p1] + nums[p2];

        if sum == target {
            return vec![(p1 + 1) as i32, (p2 + 1) as i32];
        } else if sum > target {
            p2 -= 1;
        } else {
            p1 += 1;
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solution_should_return_1_2() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 2];

        assert_eq!(solution(nums, target), expected);
    }
}
