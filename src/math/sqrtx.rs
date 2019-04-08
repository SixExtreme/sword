struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let y: i64 = x as i64;
        let mut i: i64 = 1;
        let mut j: i64 = x as i64;

        while j - i > 1 {
            if i * i == y {
                return i as i32;
            }
            if j * j == y {
                return j as i32;
            }
            let mid = i + ((j - i) / 2);
            let sqr_mid = mid * mid;
            if sqr_mid == y {
                return mid as i32;
            } else if sqr_mid < y {
                i = mid;
            } else {
                j = mid;
            }
        }
        return i as i32;
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::my_sqrt(0), 0);
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(2), 1);
    assert_eq!(Solution::my_sqrt(3), 1);
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(5), 2);
    assert_eq!(Solution::my_sqrt(6), 2);
    assert_eq!(Solution::my_sqrt(7), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(9), 3);
    assert_eq!(Solution::my_sqrt(2147395599), 46339);
}