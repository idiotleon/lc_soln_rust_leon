use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/number-of-matching-subsequences/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/number-of-matching-subsequences/discuss/117598/Java-solution-using-HashMap-and-Queue/190763
/// https://leetcode.com/problems/number-of-matching-subsequences/discuss/117598/Java-solution-using-HashMap-and-Queue
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let len_s: usize = s.len();
        let _len_ws: usize = words.len();
        let chs: Vec<char> = s.chars().collect();
        let ch_to_q: HashMap<char, RefCell<VecDeque<Vec<char>>>> = {
            let mut map: HashMap<char, RefCell<VecDeque<Vec<char>>>> =
                HashMap::with_capacity(len_s);
            for &ch in &chs {
                map.entry(ch).or_default();
            }
            for word in words {
                let chs: Vec<char> = word.chars().collect();
                if let Some(queue) = map.get(&chs[0]) {
                    queue.borrow_mut().push_back(chs);
                }
            }
            map
        };
        let mut cnt: i32 = 0;
        for ch in chs {
            if let Some(queue0) = ch_to_q.get(&ch) {
                let len_q: usize = queue0.borrow().len();
                for _ in 0..len_q {
                    let cur = queue0.borrow_mut().pop_front().unwrap_or_default();
                    if cur.len() == 1 {
                        cnt += 1;
                    } else {
                        if let Some(queue1) = ch_to_q.get(&cur[1]) {
                            queue1.borrow_mut().push_back(cur[1..].to_vec());
                        }
                    }
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let s: String = "abcde".to_owned();
        let words: Vec<String> = vec![
            "a".to_owned(),
            "bb".to_owned(),
            "acd".to_owned(),
            "ace".to_owned(),
        ];
        let expected: i32 = 3;
        let actual: i32 = Solution::num_matching_subseq(s, words);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let s: String = "dsahjpjauf".to_owned();
        let words: Vec<String> = vec![
            "ahjpjau".to_owned(),
            "ja".to_owned(),
            "ahbwzgqnuk".to_owned(),
            "tnmlanowax".to_owned(),
        ];
        let expected: i32 = 2;
        let actual: i32 = Solution::num_matching_subseq(s, words);
        assert_eq!(expected, actual);
    }
}
