/// https://leetcode.com/problems/search-suggestions-system/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/search-suggestions-system/discuss/1243383/Rust-simple-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let sorted: Vec<String> = {
            let mut tmp = products;
            tmp.sort();
            tmp
        };
        let search_word = search_word.as_bytes();
        let mut ans = vec![Vec::new(); search_word.len()];

        for product in &sorted {
            let p = product.as_bytes();
            let mut idx = 0;
            while idx < p.len().min(search_word.len()) && p[idx] == search_word[idx] {
                if ans[idx].len() < 3 {
                    ans[idx].push(product.clone())
                }
                idx += 1;
            }
        }
        ans
    }
}
