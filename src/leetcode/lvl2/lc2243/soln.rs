/// @author: Leon
/// https://leetcode.com/problems/calculate-digit-sum-of-a-string/
/// Time Complexity:    O((`len_s` ^ 2) / `k`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/calculate-digit-sum-of-a-string/discuss/1955290/Simple-Java-Solution-with-comments-for-understanding
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let _len_s: usize = s.len();
        let k: usize = k as usize;
        while s.len() > k {
            let mut new_s: String = "".to_owned();
            for idx in (0..s.len()).step_by(k) {
                let substr: &str = &s[idx..std::cmp::min(idx + k, s.len())];
                let sum_str: String = {
                    let mut sum: i32 = 0;
                    for ch in substr.chars() {
                        sum += ch as i32 - '0' as i32;
                    }
                    sum.to_string()
                };
                new_s.push_str(&sum_str);
            }
            s = new_s;
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let s: String = "11111222223".to_owned();
        let k: i32 = 3;
        let expected: String = "135".to_owned();
        let actual: String = Solution::digit_sum(s, k);
        assert_eq!(expected, actual);
    }
}
