#[allow(dead_code)]
#[derive(Default)]
struct TrieSimple {
    children: [Option<Box<TrieSimple>>; 26],
}

#[allow(dead_code)]
impl TrieSimple {
    pub fn insert(&mut self, root: &mut TrieSimple, word: String) {
        let mut cur = root;
        for &u in word.as_bytes().iter() {
            cur = cur.children[(u - b'a') as usize].get_or_insert_with(Default::default);
        }
    }
}
