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


//to do this in log(o+m) instead of merging and then searching, we must search the boundary space
//through use of a binary search meeting the conditions of placing a partition in lists A and B
//such that max_left A < min_right_B and
//          max_left B < min_right_A
//          these conditions show that the partition is placed in the same place that the median would exist in the combined array