// https://leetcode.com/problems/valid-palindrome/description/

fn solution(s: String) -> bool {
    let mut chars = s.chars().filter(|c| c.is_alphanumeric());

    while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
        if !c1.eq_ignore_ascii_case(&c2) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::s0001_valid_palindrome::solution;

    #[test]
    fn it_should_return_true() {
        let s = String::from("A man, a plan, a canal: Panama");

        assert_eq!(solution(s), true);
    }

    #[test]
    fn it_should_return_false() {
        let s = String::from("race a car");

        assert_eq!(solution(s), false);
    }
}
