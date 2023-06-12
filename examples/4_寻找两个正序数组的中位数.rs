/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let i_t = nums1.len();
        let j_t = nums2.len();
        let length = i_t + j_t;
        let pos = length / 2;
        let mut i = 0;
        let mut j = 0;
        let mut p = (0, &nums1);
        let mut last = p;
        for po in 0..pos + 1 {
            if po == pos {
                last = p.clone();
            }
            if i < i_t && j < j_t {
                if nums1[i] < nums2[j] {
                    p.0 = i;
                    p.1 = &nums1;
                    i += 1;
                } else {
                    p.0 = j;
                    p.1 = &nums2;
                    j += 1;
                }
            } else if i < i_t {
                p.0 = i;
                p.1 = &nums1;
                i += 1;
            } else {
                p.0 = j;
                p.1 = &nums2;
                j += 1;
            }
        }
        if pos % 2 == 0 {
            // 前后
            (p.1[p.0] + last.1[last.0]) as f64 / 2.
        } else {
            // 中位数
            p.1[p.0] as f64
        }
    }
}
// @lc code=end
fn main() {
    let nums = vec![
        (vec![1, 3], vec![2]), (vec![1, 2], vec![3, 4]), (vec![], vec![1])
        ];
    for n in nums {
        println!("{}", Solution::find_median_sorted_arrays(n.0, n.1));
    }
}
