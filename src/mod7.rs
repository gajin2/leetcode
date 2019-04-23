pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ret_val = 0;
        let mut abs_val = if x < 0 {
            -x
        } else {
            x
        };

        while 0 < abs_val {
            // overflow
            let old_val = ret_val;
            ret_val *= 10;
            if ret_val / 10 != old_val {
                return 0;
            }
            let remainder = abs_val % 10;
            ret_val += remainder;
            abs_val /= 10;
        }

        // overflow
        if ret_val < 0 {
            return 0;
        }

        if x < 0 {
            return -ret_val;
        }

        ret_val
    }

    pub fn test() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(21, Solution::reverse(120));
    }
}
