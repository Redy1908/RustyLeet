// https://leetcode.com/problems/product-of-array-except-self/description/

fn solution(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];

    nums.iter().enumerate().rev().fold(1, |acc, (idx, &x)| {
        result[idx] = acc;
        acc * x
    });

    nums.iter().enumerate().fold(1, |acc, (idx, &x)| {
        result[idx] *= acc;
        acc * x
    });

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_24_12_8_6() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];

        assert_eq!(solution(nums), expected);
    }
}
