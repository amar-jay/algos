
/*
 * Complete the 'reverseArray' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn reverse_array(a: &[i32]) -> Vec<i32> {

    let mut a = a.to_vec();
        a.reverse();
    a
}

/**
* Compare [i32,i32,i32] and [i32,i32,i32]
* Result: [i32, i32]
* Solution: A point is rewarded to either side if greater.
*/
fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut ans = vec![0;2];
    
    for i in 0..3 {
        if a[i] > b[i] {
            ans[0]+=1;
        }
        if a[i] < b[i] {
            ans[1]+=1;
        }
    }
    ans
}


pub fn run() {
    println!("Data Structure: Arrays");
    println!("reverseArray: {:#?}", reverse_array(&[1,2,4,8,6]));
    println!("Compare Triple: {:#?}", compare_triplets(&[1,5,7], &[1,4,6]));
}

