// https://leetcode.com/problems/longest-consecutive-sequence/description/

use std::collections::HashSet;

fn solution(nums: Vec<i32>) -> i32 {
    let set: HashSet<_> = HashSet::from_iter(nums.into_iter());
    let mut result = 0;

    for &num in set.iter() {
        if !set.contains(&(num - 1)) {
            let count = (num..).take_while(|num| set.contains(num)).count();
            result = result.max(count);
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_4() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let expected = 4;

        assert_eq!(solution(nums), expected);
    }
}
