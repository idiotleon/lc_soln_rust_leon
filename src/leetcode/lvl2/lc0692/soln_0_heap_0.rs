use std::collections::{BinaryHeap, HashMap};

/// @author: Leon
/// https://leetcode.com/problems/top-k-frequent-words/
/// Time Complexity:    O(`_len_w` * lg(`k`))
/// Space Complexity:   O(k) + O(DISTINCT(`words`))
/// Reference:
/// https://leetcode.com/problems/top-k-frequent-words/discuss/825668/Rust-tranlated-0ms-100
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let _len_w = words.len();
        let mut word_to_freq = HashMap::<String, i32>::new();
        for word in &words {
            *word_to_freq.entry(word.to_string()).or_default() += 1;
        }
        let mut heap = BinaryHeap::<(i32, String)>::new();
        for (word, freq) in word_to_freq.into_iter() {
            heap.push((-freq, word));
            if heap.len() > (k as usize) {
                heap.pop();
            }
        }
        let mut ans: Vec<String> = vec![];
        while let Some((_freq, word)) = heap.pop() {
            ans.push(word);
        }
        ans.reverse();
        ans
    }
}
