// binary search
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() -1;
    let mut mid = 0;
    while left < right {
        mid = left+(right-left)/2;
        if arr[mid-1] < arr[mid] && arr[mid] > arr[mid+1] {
            break;
        }
        if arr[mid] < arr[mid+1] {
            left = mid;
        }
        if arr[mid] > arr[mid+1] {
            right = mid;
        }
    }
    mid as i32
}

fn main() {
    let arr = vec![18,29,38,59,98,100,99,98,90];
    println!("{:?}", peak_index_in_mountain_array(arr));
}
