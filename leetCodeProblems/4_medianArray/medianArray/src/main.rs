fn main() {
    let answer = find_median_sorted_arrays(vec![1,2,3,4,5], vec![3,4]);
    println!("{}",answer);
}
/*
//O(m+n) implementation
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut result = Vec::with_capacity(nums1.len() + nums2.len());
    let mut i = 0;
    let mut j = 0;
    if nums1.len() > 0 && nums2.len() > 0{
        while i <= nums1.len() -1 && j <= nums2.len() -1{
            if nums1[i] <= nums2[j] {
                result.push(nums1[i]);
                i += 1;
            } else {
                result.push(nums2[j]);
                j += 1;
            }    
        }
    }
    result.extend_from_slice(&nums1[i..]);
    result.extend_from_slice(&nums2[j..]);
    
    if result.len() % 2 == 0 {
        return (result[result.len()/2 -1] as f64 + result[result.len()/2] as f64)/2.0;
    }
    return result[result.len()/2] as f64;
}
*/
/*
//regular binary search
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut low = 0;
    let mut high = nums1.len() - 1; //this needs to be length of smaller when using both arrays
    let mut mid = 0;
    let target = 6;

    while low <= high {
        mid = (low + high) / 2;
        if nums1[mid] == target {
            println!("found target {}",nums1[mid]);
            return target as f64;
        }
        else if nums1[mid] < target {
            low = mid + 1; //too small, search right side
            println!("too small {}",nums1[mid]);
        }
        else { //too big, go to left half
            high = mid - 1;
            println!("too big {}",nums1[mid]);

        }
    }
    return -1.0;
}
*/
//log(m+n) search
use std::i32::{MIN, MAX};
use std::cmp::{max, min};
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };
    let m = a.len();
    let n = b.len();
    let half_len = (m + n + 1) / 2;

    let mut low = 0;
    let mut high = m;

    while low <= high {
        let i = (low + high) / 2;
        let j = half_len - i;

        //set to MIN if smaller partition has no elements, or MAX if larger has no elements, otherwise set it to the desired
        let a_left = if i == 0 { MIN } else { a[i - 1] };
        let a_right = if i == m { MAX } else { a[i] };
        let b_left = if j == 0 { MIN } else { b[j - 1] };
        let b_right = if j == n { MAX } else { b[j] };
        //check that the median condition is met, then return appropriate value for even/odd cases
        if a_left <= b_right && b_left <= a_right {
            if (m + n) % 2 == 0 {
                return (f64::from(max(a_left, b_left)) +
                        f64::from(min(a_right, b_right))) / 2.0;
            } else {
                return f64::from(max(a_left, b_left));
            }
        } else if a_left > b_right {
            high = i - 1;
        } else {
            low = i + 1;
        }
    }
    return -1.0;
}