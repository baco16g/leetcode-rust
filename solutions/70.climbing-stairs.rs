/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // if n == 1 {
        //     return 1;
        // }
        // let mut dp = vec![0, 1, 2];
        // for i in 3..(n + 1) {
        //     dp.insert(
        //         i as usize,
        //         dp.get((i - 1) as usize).unwrap() + dp.get((i - 2) as usize).unwrap(),
        //     );
        // }
        // *dp.get(n as usize).unwrap()

        if n == 1 {
            return 1;
        }
        let mut first = 1;
        let mut second = 2;
        for _ in 3..(n + 1) {
            let sum = first + second;
            first = second;
            second = sum;
        }
        second
    }
}
// @lc code=end
