use std::cell::RefCell;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s` ^ 2)
/// Reference:
/// https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/discuss/1759403/Java-rolling-hash-13-lines-%2B-trie
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_digit_frequency(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut ans: i32 = 0;
        // dummy root
        let root = Rc::new(RefCell::new(TrieNode::new()));
        for lo in 0..len_s {
            let mut freqs: Vec<u16> = vec![0; 10];
            let mut unique: u16 = 0;
            let mut most: u16 = 0;
            let mut cur = Some(root.clone());
            for hi in lo..len_s {
                let digit: usize = chs[hi] as usize - '0' as usize;
                if freqs[digit] == 0 {
                    unique += 1;
                }
                freqs[digit] += 1;
                most = std::cmp::max(most, freqs[digit]);
                let tmp = cur.clone().unwrap();
                let children = &mut tmp.borrow_mut().children;
                if children[digit].is_none() {
                    children[digit] = Some(Rc::new(RefCell::new(TrieNode::new())));
                }
                cur = children[digit].clone();
                if unique * most == (hi - lo + 1) as u16 {
                    let tmp = cur.clone().unwrap();
                    let seen = &mut tmp.borrow_mut().seen;
                    if !*seen {
                        ans += 1;
                        *seen = true;
                    }
                }
            }
        }
        ans
    }
}

struct TrieNode {
    children: Vec<Option<Rc<RefCell<TrieNode>>>>,
    seen: bool,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            children: vec![None; 10],
            seen: false,
        }
    }
}
