use std::collections::BTreeMap;

/// @author: Leon
/// https://leetcode.com/problems/snapshot-array/
/// Time Complexities:
///         `new`:      O(`length`)
///         `set`:      O(1)
///         `snap`:     O(1)
///         `get`:      O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/snapshot-array/discuss/350562/JavaPython-Binary-Search
#[allow(dead_code)]
struct SnapshotArray {
    maps: Vec<BTreeMap<i32, i32>>,
    snap_id: i32,
}

#[allow(dead_code)]
impl SnapshotArray {
    fn new(length: i32) -> Self {
        let length: usize = length as usize;
        Self {
            maps: vec![BTreeMap::new(); length]
                .into_iter()
                .map(|mut e| {
                    e.entry(0).or_insert(0);
                    e
                })
                .collect(),
            snap_id: 0,
        }
    }
    fn set(&mut self, index: i32, val: i32) {
        self.maps[index as usize].insert(self.snap_id, val);
    }
    fn snap(&mut self) -> i32 {
        let id = self.snap_id;
        self.snap_id += 1;
        id
    }
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        *self.maps[index as usize]
            .range(..=snap_id)
            .next_back()
            .unwrap()
            .1
    }
}
