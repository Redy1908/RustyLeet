// https://leetcode.com/problems/container-with-most-water/description/

use std::cmp::{max, min};

fn solution(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left = 0;
    let mut rigth = height.len() - 1;

    while left < rigth {
        let area = (rigth - left) as i32 * min(height[left], height[rigth]);
        result = max(result, area);

        if height[left] < height[rigth] {
            left += 1;
        } else {
            rigth -= 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_49() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        assert_eq!(solution(height), expected);
    }
}
