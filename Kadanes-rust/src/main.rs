fn kadanes(arr: &[i32]) -> i32{
    let mut current_max = arr[0];
    let mut final_max = arr[0];

    for &num in arr.iter().skip(1){
        current_max = i32::max(num, num+current_max);

        final_max = i32::max(final_max, current_max);

    }
    final_max
}

fn main() {
    let arr = [-2,-3,4,-1,-2,1,5,-3];
    let max_sum = kadanes(&arr);
    println!("The max sum is: {}",max_sum);
}
