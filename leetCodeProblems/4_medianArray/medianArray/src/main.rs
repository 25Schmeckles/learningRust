fn main() {
    let answer = find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>);
    println!("Hello, world!");
}
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        return 2;
    }

//to do this in log(o+m) instead of merging and then searching, we must search the boundary space
//through use of a binary search meeting the conditions of placing a partition in lists A and B
//such that max_left A < min_right_B and
//          max_left B < min_right_A
//          these conditions show that the partition is placed in the same place that the median would exist in the combined array