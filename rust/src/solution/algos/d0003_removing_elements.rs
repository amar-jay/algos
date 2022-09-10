

#[allow(unused)]
fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut arr= arr.to_vec();

   
    let mut x = 0;
    arr.retain(|v| {
        x += 1;
        if x%2==0 {
            return false;
        }
        true
    });
    arr
    }
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_every_other;

   #[test]
    fn sample_test() {
        assert_eq!(remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), &[1, 3, 5, 7, 9]);
    }
}
