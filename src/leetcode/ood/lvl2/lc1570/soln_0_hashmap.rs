use std::collections::HashMap;

/// https://leetcode.com/problems/dot-product-of-two-sparse-vectors/
/// Time Complexity:    O(`_len_n1` & `_len_n2`)
/// Space Complexity:   O(`_len_n1` + `_len_n2`) ~ O(max(`_len_n1`, `_len_n2`))
#[allow(dead_code)]
struct SparseVector {
    idx_to_num: HashMap<usize, i32>,
}

#[allow(dead_code)]
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let _len_n: usize = nums.len();
        let idx_to_num: HashMap<usize, i32> = {
            let mut idx_to_num: HashMap<usize, i32> = HashMap::new();
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
