
/*
 * Complete the 'hourglassSum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */
fn hourglasssum(arr: &[Vec<i32>]) -> i32 {
    let mut max = -63;
    for i in 0..4 {
        for j in 0..4 {
            let sum = arr[i][j] + arr[i][j + 1] + arr[i][j + 2] + arr[i + 1][j + 1] + arr[i + 2][j] + arr[i + 2][j + 1] + arr[i + 2][j + 2];

            if max < sum {
                max = sum;
            }
        }
    }

    max

}

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let len = arr.len();
    let mut ans = 0;
    for (idx, _) in arr.iter().enumerate() {
        let lhs = arr[idx][idx];
        let rhs = arr[len-idx-1][idx];
        ans += lhs-rhs
    }
    ans.abs()
}




pub fn run() {
   println!("Data Structure: 2D Arrays");
    println!("hourglassSum: {:#}", hourglasssum(&[
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6]
    ]));
    println!("diagonal difference: {:#?}", diagonal_difference(&[
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,4,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6],
        vec![1,3,4,5,7,6]
 
    ]));
}
