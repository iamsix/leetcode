impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for idx in 0..nums.len() {
            for idx2 in idx+1..nums.len() {
                if nums[idx] + nums[idx2] == target {
                    return vec![idx as i32, idx2 as i32];
                }
            }
        }
        return vec![0 as i32];
    }
    
}
