/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut current_max = 0;
        for n in nums {
            current_max = std::cmp::max(n, current_max + n);
            max = std::cmp::max(max, current_max);
        }
        max
    }
}
// @lc code=end
