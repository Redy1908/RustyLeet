// https://leetcode.com/problems/encode-and-decode-strings/description/

fn solution(strs: Vec<&str>) -> Vec<String> {
    let encoded = encode(strs);
    decode(&encoded)
}

fn encode(strs: Vec<&str>) -> String {
    strs.join("#")
}

fn decode(str: &str) -> Vec<String> {
    str.split("#").map(String::from).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_true() {
        let strs = vec!["lint", "code", "love", "you"];
        let expected = vec!["lint", "code", "love", "you"];
        assert_eq!(solution(strs), expected);
    }
}
