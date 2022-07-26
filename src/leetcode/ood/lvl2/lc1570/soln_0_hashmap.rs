use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/dot-product-of-two-sparse-vectors/
/// Time Complexity:    O(`_len_ns1` & `_len_ns2`)
/// Space Complexity:   O(`_len_ns1` + `_len_ns2`) ~ O(max(`_len_ns1`, `_len_ns2`))
struct SparseVector {
    idx_to_num: HashMap<usize, i32>,
}

#[allow(dead_code)]
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let len_ns: usize = nums.len();
        let idx_to_num: HashMap<usize, i32> = {
            let mut idx_to_num: HashMap<usize, i32> = HashMap::with_capacity(len_ns);
            for (idx, num) in nums.into_iter().enumerate() {
                if num != 0 {
                    idx_to_num.insert(idx, num);
                }
            }
            idx_to_num
        };
        SparseVector { idx_to_num }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut product: i32 = 0;
        for (&idx, &num1) in &self.idx_to_num {
            if let Some(&num2) = vec.idx_to_num.get(&idx) {
                product += num1 * num2;
            }
        }
        product
    }
}
