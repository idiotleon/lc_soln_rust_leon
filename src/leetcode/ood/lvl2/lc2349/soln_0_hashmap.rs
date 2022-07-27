use std::collections::{BTreeSet, HashMap};

/// @author: Leon
/// https://leetcode.com/problems/design-a-number-container-system/
/// Time Complexities:
///         `new()`:    O(1)
///         `change()`: O(1)
///         `find()`:   O(1)
/// Space Complexity:   O(N)
#[allow(dead_code)]
struct NumberContainers {
    num_to_indices: HashMap<i32, BTreeSet<i32>>,
    idx_to_num: HashMap<i32, i32>,
}

#[allow(dead_code)]
impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            num_to_indices: HashMap::new(),
            idx_to_num: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&num) = self.idx_to_num.get(&index) {
            self.num_to_indices.entry(num).or_default().remove(&index);
        }
        self.num_to_indices.entry(number).or_default().insert(index);
        self.idx_to_num.insert(index, number);
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(indices) = self.num_to_indices.get(&number) {
            *indices.iter().next().unwrap_or(&-1)
        } else {
            -1
        }
    }
}
