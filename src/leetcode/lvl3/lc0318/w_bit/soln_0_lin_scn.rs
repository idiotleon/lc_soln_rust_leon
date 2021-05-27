/// https://leetcode.com/problems/maximum-product-of-word-lengths/
///
/// Time Complexity:    O(`len_ws` ^ 2)
/// Space Complexity:   O(`len_ws`)
///
/// Reference:
/// https://leetcode.com/problems/maximum-product-of-word-lengths/discuss/76959/JAVA-Easy-Version-To-Understand!!!!!!!!!!!!!!!!!
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let len_ws = words.len();

        let bit_masks: Vec<u32> = {
            let mut bit_masks = vec![0; len_ws];

            for (idx, word) in words.iter().enumerate() {
                for ch in word.chars() {
                    bit_masks[idx] |= 1 << (ch as u32 - 'a' as u32);
                }
            }

            bit_masks
        };

        let mut max_product: i32 = 0;
        for lo in 0..len_ws {
            for hi in lo + 1..len_ws {
                if bit_masks[lo] & bit_masks[hi] == 0 {
                    max_product = std::cmp::max(
                        max_product,
                        (words[lo].len() as i32) * (words[hi].len() as i32),
                    );
                }
            }
        }

        max_product
    }
}
