use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() as i32 {
            let comp = target - nums[i as usize];
            if map.contains_key(&comp) {
                return vec![map[&comp], i];
            }
            map.insert(nums[i as usize], i);
        }
        
        panic!("No solution found!")
    }
    
}
