/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut i = (a.len() - 1) as i32;
        let mut j = (b.len() - 1) as i32;
        let mut carry = 0;

        while i >= 0 || j >= 0 {
            let mut sum = carry;
            sum += a.chars().nth(i as usize).map_or(0, |x| x as i32 - 48);
            sum += b.chars().nth(j as usize).map_or(0, |x| x as i32 - 48);
            result.push_str(&(sum % 2).to_string());
            carry = sum / 2;

            i -= 1;
            j -= 1;
        }

        if carry != 0 {
            result.push_str(&carry.to_string());
        }
        result.chars().rev().collect()
    }
}
// @lc code=end
