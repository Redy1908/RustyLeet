// https://leetcode.com/problems/top-k-frequent-elements/description/

use std::collections::HashMap;

fn solution(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    nums.into_iter()
        .for_each(|num| *map.entry(num).or_insert(0) += 1);
    let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec.iter().take(k as usize).map(|x| x.0).collect()
}

#[cfg(test)]
mod test {
    use crate::solutions::s0005_top_k_frequent_elements::solution;

    #[test]
    fn it_should_return_1_and_2() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let expected = vec![1, 2];

        assert_eq!(solution(nums, k), expected);
    }

    #[test]
    fn it_should_return_1() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];

        assert_eq!(solution(nums, k), expected);
    }
}
