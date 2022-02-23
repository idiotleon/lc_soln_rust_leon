use std::cell::RefCell;
use std::rc::Rc;
/// @author: Leon
/// https://leetcode.com/problems/word-search-ii/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// This is NOT a correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const IMPL: char = '#';
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let len_r: usize = board.len();
        let len_c: usize = board[0].len();
        let root = Self::build_trie(words);
        let mut ans: Vec<String> = Vec::new();
        for r in 0..len_r {
            for c in 0..len_c {
                Self::backtrack((r as isize, c as isize), &mut board, root.clone(), &mut ans);
            }
        }
        ans
    }
    fn backtrack(
        coord: (isize, isize),
        board: &mut Vec<Vec<char>>,
        mut node: Rc<RefCell<TrieNode>>,
        res: &mut Vec<String>,
    ) {
        let len_r: usize = board.len();
        let len_c: usize = board[0].len();
        let (r, c) = coord;
        if r < 0 || r >= len_r as isize || c < 0 || c >= len_c as isize {
            return;
        }
        let r: usize = r as usize;
        let c: usize = c as usize;
        let hold = board[r][c];
        let idx_ch: usize = hold as usize - 'a' as usize;
        if hold == Self::IMPL || node.borrow().children[idx_ch].clone().is_none() {
            return;
        }
        // temp variable, cannot be omitted
        let tmp = node.clone();
        let children = &tmp.borrow().children;
        node = children[idx_ch].clone().unwrap();
        if node.borrow().word.is_some() {
            res.push(node.borrow().word.clone().unwrap().to_owned());
            node.borrow_mut().word = None;
        }
        board[r][c] = Self::IMPL;
        for d in 0..4 {
            let r_nxt: isize = r as isize + Self::DIRS[d];
            let c_nxt: isize = c as isize + Self::DIRS[d + 1];
            Self::backtrack((r_nxt, c_nxt), board, node.clone(), res);
        }
        board[r][c] = hold;
    }
    fn build_trie(words: Vec<String>) -> Rc<RefCell<TrieNode>> {
        let root: Rc<RefCell<TrieNode>> = {
            let root: Rc<RefCell<TrieNode>> = Rc::new(RefCell::new(TrieNode::new(None)));
            let mut cur = Some(root.clone());
            for word in words {
                for ch in word.chars() {
                    let idx_ch = ch as usize - 'a' as usize;
                    let tmp = cur.unwrap().clone();
                    let children = &mut tmp.borrow_mut().children;
                    if children[idx_ch].is_none() {
                        children[idx_ch] = Some(Rc::new(RefCell::new(TrieNode::new(None))));
                    }
                    cur = children[idx_ch].clone();
                }
                cur.clone().unwrap().borrow_mut().word = Some(word);
            }
            root
        };
        root
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
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words: Vec<String> = vec![
            "oath".to_owned(),
            "pea".to_owned(),
            "eat".to_owned(),
            "rain".to_owned(),
        ];
        let actual = Solution::find_words(board, words);
        let expected = vec!["eat".to_owned(), "oath".to_owned()];
        assert_eq!(expected, actual);
    }
}
