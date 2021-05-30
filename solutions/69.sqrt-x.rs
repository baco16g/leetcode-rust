/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0;
        let mut high = x;
        let mut mid = 0;

        if x == 0 || x == 1 {
            return x;
        }

        while low <= high {
            mid = (low + high) / 2;
            if high - low == 1 {
                return mid;
            } else if mid < x / mid {
                low = mid;
            } else if x / mid < mid {
                high = mid;
            } else {
                return mid;
            }
        }
        mid
    }
}
// @lc code=end
