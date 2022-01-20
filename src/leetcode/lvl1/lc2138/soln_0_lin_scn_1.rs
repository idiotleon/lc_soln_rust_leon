/// @author: Leon
/// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let len_s: usize = s.len();
        let ans: Vec<String> = {
            let mut res: Vec<String> = Vec::new();
            let mut str: String = "".to_owned();
            for (idx, ch) in s.chars().enumerate() {
                str.push(ch);
                if (1 + idx as i32) % k == 0 {
                    res.push(str.to_owned());
                    str.clear();
                }
            }
            let modulus: usize = len_s % k as usize;
            if modulus > 0 {
                let times = k as usize - modulus;
                for _ in 0..times {
                    str.push(fill);
                }
                res.push(str.to_owned());
            }
            res
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s: String = "abcdefghi".to_owned();
        let k = 3;
        let fill: char = 'x';
        let actual = Solution::divide_string(s, k, fill);
        let expected: Vec<String> = vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s: String = "abcdefghij".to_owned();
        let k = 3;
        let fill: char = 'x';
        let actual = Solution::divide_string(s, k, fill);
        let expected: Vec<String> = vec![
            "abc".to_owned(),
            "def".to_owned(),
            "ghi".to_owned(),
            "jxx".to_owned(),
        ];
        assert_eq!(expected, actual);
    }
}
