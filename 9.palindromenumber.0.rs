impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut nums = Vec::new();
        let mut x = x;
        while x != 0 {
            let val = x % 10;
            nums.push(val);
            x /= 10;
        }
        
        let mut rev = nums.clone();
        rev.reverse();
        if rev == nums {
            true
        } else {
            false
        }
    }
}
