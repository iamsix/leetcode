use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut idx1 = 0;
        let mut idx2 = 0;
        let ln1 = nums1.len();
        let ln2 = nums2.len();
        let mut nums = Vec::new();

        while idx1 < ln1 || idx2 < ln2 {
            if idx1 < ln1 && idx2 < ln2 && nums1[idx1] <= nums2[idx2]  {
                nums.push(nums1[idx1]);
                idx1 += 1;
            } else if idx1 < ln1 && idx2 == ln2 {
                nums.push(nums1[idx1]);
                idx1 += 1;
            } else if idx2 < ln2 && idx1 < ln1 && nums2[idx2] <= nums1[idx1] {
                nums.push(nums2[idx2]);
                idx2 += 1;
            } else if idx2 < ln2 && idx1 == ln1 {
                nums.push(nums2[idx2]);
                idx2 += 1;
            }
        }
        
        let half = nums.len() / 2;
        if nums.len() % 2 == 1 {
            nums[half] as f64
        } else {
            (nums[half - 1] + nums[half]) as f64 / 2.0
        }
        
    }
}
