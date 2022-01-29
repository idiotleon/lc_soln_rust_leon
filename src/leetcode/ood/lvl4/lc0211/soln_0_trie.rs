use std::cell::RefCell;
use std::rc::Rc;
/// @author: Leon
/// https://leetcode.com/problems/design-add-and-search-words-data-structure/
/// Time Complexities:
///     `new()`:        O(1)
///     `add_word()`:   O(`_len_wd`)
///     `search()`:     O(`_len_wd`)
/// Space Complexity:   O(n * `_len_wd`)
struct WordDictionary {
    root: Rc<RefCell<TrieNode>>,
}

#[allow(dead_code)]
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Rc::new(RefCell::new(TrieNode::new(None))),
        }
    }

    fn add_word(&mut self, word: String) {
        let _len_wd: usize = word.len();
        let mut cur = Some(self.root.clone());
        for ch in word.chars() {
            let idx_ch: usize = ch as usize - 'a' as usize;
            let tmp = cur.unwrap().clone();
            let children = &mut tmp.borrow_mut().children;
            if children[idx_ch].is_none() {
                children[idx_ch] = Some(Rc::new(RefCell::new(TrieNode::new(None))));
            }
            cur = children[idx_ch].clone();
        }
        cur.unwrap().borrow_mut().word = Some(word);
    }

    fn search(&self, word: String) -> bool {
        let _len_wd: usize = word.len();
        Self::dfs(0, &word.chars().collect(), self.root.clone())
    }

    fn dfs(idx: usize, chs_wd: &Vec<char>, node: Rc<RefCell<TrieNode>>) -> bool {
        let len_w: usize = chs_wd.len();
        const ANY: char = '.';
        if idx == len_w {
            return node.borrow().word.is_some();
        }
        let ch = chs_wd[idx];
        match ch {
            ANY => {
                for ele in node.borrow().children.iter() {
                    if let Some(child) = ele.clone() {
                        if Self::dfs(idx + 1, chs_wd, child) {
                            return true;
                        }
                    }
                }
            }
            _ => match node.borrow().children[ch as usize - 'a' as usize].clone() {
                Some(child) => {
                    return Self::dfs(idx + 1, chs_wd, child);
                }
                None => return false,
            },
        }
        false
    }
}

struct TrieNode {
    word: Option<String>,
    children: Vec<Option<Rc<RefCell<TrieNode>>>>,
}

impl TrieNode {
    fn new(word: Option<String>) -> Self {
        TrieNode {
            word,
            children: vec![None; 26],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let word_dict: WordDictionary = {
            let mut word_dict = WordDictionary::new();
            word_dict.add_word("bad".to_owned());
            word_dict.add_word("dad".to_owned());
            word_dict.add_word("mad".to_owned());
            word_dict
        };
        let actual0: bool = word_dict.search("pad".to_owned());
        let expected0: bool = false;
        assert_eq!(expected0, actual0);
        let actual1: bool = word_dict.search("bad".to_owned());
        let expected1: bool = true;
        assert_eq!(expected1, actual1);
        let actual2: bool = word_dict.search(".ad".to_owned());
        let expected2: bool = true;
        assert_eq!(expected2, actual2);
        let actual3: bool = word_dict.search("b..".to_owned());
        let expected3: bool = true;
        assert_eq!(expected3, actual3);
    }
}
