// https://leetcode.com/problems/prefix-and-suffix-search/
//
// Time Complexities:
//  `new()`:    O()
//  `f()`:      O()
//
// Space Complexity:    O()
//
// Reference:
//  https://leetcode.com/problems/prefix-and-suffix-search/discuss/1185417/Rust-Trie-solution
#[derive(Default)]
struct Trie {
    idx: i32,
    children: [Option<Box<Trie>>; 27],
}

struct WordFilter {
    root: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut root = Trie::default();
        for (idx, word) in words.iter().enumerate() {
            let len = word.len();
            let str = String::new() + &word + "{" + &word;
            for j in 0..len {
                let mut node = &mut root;
                for &b in &str.as_bytes()[j..] {
                    node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
                    node.idx = idx as i32;
                }
            }
        }
        Self { root }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut cur = &self.root;
        let str = String::new() + &suffix + "{" + &prefix;
        for &b in str.as_bytes() {
            if let Some(node) = &cur.children[(b - b'a') as usize] {
                cur = node.as_ref();
            } else {
                return -1;
            }
        }

        cur.idx
    }
}
