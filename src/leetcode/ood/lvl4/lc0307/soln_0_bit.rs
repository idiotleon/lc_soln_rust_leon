/// @author: Leon
/// https://leetcode.com/problems/range-sum-query-mutable/
/// Time Complexities:
///         `new()`:        O(`_len_ns` * lg(`_len_ns`))
///         `update()`:     O(lg(`_len_ns`))
///         `sum_range()`:  O(lg(`_len_ns`))
/// Space Complexity:       O(`_len_ns`)
/// Reference:
/// https://leetcode.com/problems/range-sum-query-mutable/discuss/75753/Java-using-Binary-Indexed-Tree-with-clear-explanation
/// https://cs.stackexchange.com/questions/10538/bit-what-is-the-intuition-behind-a-binary-indexed-tree-and-how-was-it-thought-a
#[allow(dead_code)]
struct NumArray {
    nums: Vec<i32>,
    bit: BinaryIndexedTree,
}

#[allow(dead_code)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let _len_ns: usize = nums.len();
        let bit: BinaryIndexedTree = BinaryIndexedTree::new(&nums);
        Self { nums, bit }
    }

    fn update(&mut self, index: i32, val: i32) {
        let idx: usize = index as usize;
        let diff = val - self.nums[idx];
        self.nums[idx] = val;
        self.bit.update(idx as isize, diff);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.bit.query(right as isize) - self.bit.query(left as isize - 1)
    }
}

struct BinaryIndexedTree {
    _len_ns: isize,
    fenwick: Vec<i32>,
}

impl BinaryIndexedTree {
    pub fn new(nums: &Vec<i32>) -> Self {
        let _len_ns: usize = nums.len();
        let fenwick: Vec<i32> = vec![0; _len_ns + 1];
        let mut obj = Self {
            _len_ns: _len_ns as isize,
            fenwick,
        };
        for (idx, &num) in nums.iter().enumerate() {
            obj.update(idx as isize, num)
        }
        obj
    }
    pub fn update(&mut self, index: isize, value: i32) {
        let mut idx = index + 1;
        while idx <= self._len_ns {
            self.fenwick[idx as usize] += value;
            idx += idx & -idx;
        }
    }
    pub fn query(&self, index: isize) -> i32 {
        let mut sum: i32 = 0;
        let mut idx = index + 1;
        while idx > 0 {
            sum += self.fenwick[idx as usize];
            idx -= idx & -idx;
        }
        sum
    }
}
