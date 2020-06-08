use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        let mut nums2 = nums2;
        nums.append(&mut nums2);
        nums.sort();
        
        let ln = nums.len();
        if ln % 2 == 1 {
            nums[ln / 2] as f64
        } else {
            (nums[(ln / 2) - 1] + nums[ln / 2]) as f64 / 2.0
        }
        
    }
}
