pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..height.len() {
            for j in (i+1)..height.len() {
                let area = std::cmp::min(height[i], height[j]) * (j as i32 - i as i32);
                if ret < area {
                    ret = area;
                }
            }
        }
        ret
    }

    pub fn test() {
        assert_eq!(49, Solution::max_area([1,8,6,2,5,4,8,3,7].to_vec()));
    }
}
