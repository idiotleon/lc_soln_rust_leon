use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/remove-k-digits/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let len_s: usize = num.len();
        if len_s <= k as usize {
            return "0".to_owned();
        }
        let mut stk: VecDeque<u8> = VecDeque::new();
        let mut cnt: i32 = 0;
        for ch in num.chars() {
            let digit = ch as u8 - '0' as u8;
            'inner: while let Some(&top) = stk.back() {
                if cnt >= k || top <= digit {
                    break 'inner;
                }
                stk.pop_back();
                cnt += 1;
            }
            stk.push_back(digit);
        }
        while cnt < k {
            stk.pop_back();
            cnt += 1;
        }
        // to construct the answer string
        let mut chs: Vec<char> = Vec::with_capacity(stk.len());
        while let Some(top) = stk.pop_back() {
            chs.push((top + '0' as u8) as char);
        }
        // to remove the trailing 0s
        while let Some(&ch) = chs.last() {
            if ch == '0' {
                chs.pop();
            } else {
                break;
            }
        }
        chs.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let num: String = "1432219".to_owned();
        let k: i32 = 3;
        let actual = Solution::remove_kdigits(num, k);
        let expected = "1219".to_owned();
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_26() {
        let num: String = "112".to_owned();
        let k: i32 = 1;
        let actual: String = Solution::remove_kdigits(num, k);
        let expected: String = "11".to_owned();
        assert_eq!(expected, actual);
    }
}
