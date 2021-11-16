/// https://leetcode.com/problems/iterator-for-combination/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/iterator-for-combination/discuss/778870/Rust
#[allow(dead_code)]
struct CombinationIterator {
    pool: Vec<String>,
    current: usize,
}

#[allow(dead_code)]
impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let chs = characters.chars().collect::<Vec<char>>();
        let len_c = chs.len();
        let power2_of_n = {
            let mut power2_of_n = 1usize;
            for _ in 1..=len_c as usize {
                power2_of_n *= 2;
            }
            power2_of_n
        };
        let pool: Vec<String> = {
            let mut pool: Vec<String> = Vec::new();
            for i in 1..power2_of_n {
                if i.count_ones() == combination_length as u32 {
                    let mut s = String::new();
                    let mut j = 1usize;
                    let mut mask = 1;
                    while mask <= power2_of_n as usize {
                        if i & mask > 0 {
                            s.push(chs[j - 1]);
                        }
                        j += 1;
                        mask <<= 1;
                    }
                    pool.push(s);
                }
            }
            pool.sort();
            pool
        };
        CombinationIterator { pool, current: 0 }
    }
    fn next(&mut self) -> String {
        self.current += 1;
        self.pool[self.current - 1].clone()
    }
    fn has_next(&self) -> bool {
        self.current < self.pool.len()
    }
}
