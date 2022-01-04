use std::collections::{HashMap, VecDeque};
/// https://leetcode.com/problems/longest-common-prefix/
/// Time Complexity:    O(`_len_ws` * avg_len_word)
/// Space Complexity:   O(`_len_ws` * avg_len_word)
/// Reference:
/// https://iq.opengenus.org/trie-in-rust/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> String {
        let _len_ws: usize = words.len();
        let trie: Trie = {
            let mut root = Trie::new(None, None);
            for word in words {
                root.insert(&word);
            }
            root
        };
        let ans: String = {
            let mut res: String = "".to_owned();
            let mut queue: VecDeque<TrieNode> = VecDeque::new();
            queue.push_back(trie.root);
            'outer: while !queue.is_empty() {
                let len_q: usize = queue.len();
                if len_q > 1 {
                    break 'outer;
                }
                for _ in 0..len_q {
                    let cur = queue.pop_front().unwrap();
                    if let Some(ch) = cur.ch_op {
                        res.push(ch);
                    }
                    for (_, node) in cur.ch_to_node.into_iter() {
                        queue.push_back(node);
                    }
                    if let Some(_word) = cur.word_op {
                        break 'outer;
                    }
                }
            }
            res
        };
        ans
    }
}

struct TrieNode {
    ch_op: Option<char>,
    word_op: Option<String>,
    ch_to_node: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(ch_op: Option<char>, word_op: Option<String>) -> TrieNode {
        TrieNode {
            ch_op,
            word_op,
            ch_to_node: HashMap::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new(ch_op: Option<char>, word_op: Option<String>) -> Trie {
        Trie {
            root: TrieNode {
                ch_op,
                word_op,
                ch_to_node: HashMap::new(),
            },
        }
    }
    fn insert(&mut self, s: &str) {
        let mut cur = &mut self.root;
        for ch in s.chars() {
            cur = Self::moving(cur)
                .ch_to_node
                .entry(ch)
                .or_insert(TrieNode::new(Some(ch), None));
        }
        cur.word_op = Some(s.to_owned());
    }
    #[allow(dead_code)]
    fn find(&self, s: &str) -> bool {
        let mut cur = &self.root;
        for ch in s.chars() {
            if let Some(nxt) = cur.ch_to_node.get(&ch) {
                cur = nxt;
            } else {
                return false;
            }
        }
        if let Some(word) = &cur.word_op {
            word.eq(s)
        } else {
            false
        }
    }
    fn moving<T>(t: T) -> T {
        t
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let strs = vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()];
        let actual = Solution::longest_common_prefix(strs);
        let expected = "fl".to_owned();
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2(){
        let strs = vec!["dog".to_owned(), "racecar".to_owned(), "car".to_owned()];
        let actual = Solution::longest_common_prefix(strs);
        let expected = "";
        assert_eq!(expected, actual);
    }
}
