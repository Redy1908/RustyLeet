// https://leetcode.com/problems/3sum/solutions/

use core::num;

fn solution(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    nums.sort();

    for i in 0..nums.len() - 1 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let tree_sum = nums[i] + nums[left] + nums[right];

            if tree_sum > 0 {
                right -= 1;
            } else if tree_sum < 0 {
                left += 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);

                left += 1;

                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::s0003_3_sum::solution;

    #[test]
    fn should_return_true() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(solution(nums), expected);
    }
}
