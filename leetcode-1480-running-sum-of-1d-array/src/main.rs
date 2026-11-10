fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = running_sum(nums);
    println!("{:?}", result);
}

fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    let mut current_sum = 0;
    for num in nums.iter_mut() {
        current_sum += *num;
        *num = current_sum;
    }

    nums
}