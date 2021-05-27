/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for index in (0..digits.len()).rev() {
            if let Some(v) = digits.get_mut(index) {
                if *v == 9 {
                    *v = 0;
                } else {
                    *v += 1;
                    return digits;
                }
            }
        }
        digits.insert(0, 1);
        digits
    }
}
// @lc code=end
