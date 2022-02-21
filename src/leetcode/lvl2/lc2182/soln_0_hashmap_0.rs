/// @author: Leon
/// https://leetcode.com/problems/construct-string-with-repeat-limit/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/construct-string-with-repeat-limit/discuss/1784718/C++-Greedy-+-Counting-O(N)-Time-O(1)-space/1275778
/// https://leetcode.com/problems/construct-string-with-repeat-limit/discuss/1784718/C%2B%2B-Greedy-%2B-Counting-O(N)-Time-O(1)-space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let _len_s: usize = s.len();
        let mut freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 26];
            for ch in s.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut ans: String = "".to_owned();
        let mut found: bool = true;
        let mut cnt: i32 = 0;
        let mut idx_last: usize = 27;
        while found {
            found = false;
            for idx in (0..26).rev() {
                if found {
                    break;
                }
                if freqs[idx] > 0 && (cnt < repeat_limit || idx_last != idx) {
                    ans.push(('a' as u8 + idx as u8) as char);
                    cnt += 1;
                    freqs[idx] -= 1;
                    if idx_last != idx {
                        cnt = 1;
                    }
                    idx_last = idx;
                    found = true;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s: String = "cczazcc".to_owned();
        let repeat_limit: i32 = 3;
        let actual = Solution::repeat_limited_string(s, repeat_limit);
        let expected = "zzcccac".to_owned();
        assert_eq!(expected, actual);
    }
}
