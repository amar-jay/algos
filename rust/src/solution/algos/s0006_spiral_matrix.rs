pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut vec:Vec<Vec<i32>> = vec![vec![0;n];n];
    let (x_min, x_max, y_min, y_max) = (0, n-1, 0, n-1);
    for i in 0..(n*n) {
        if i < x_max {
            vec[x_min][i] = i as i32;
            continue
        }

        if i > x_max &&  i-x_max < y_max{ 
            vec[x_max][i%n] = i as i32;
        }

        if i-x_max-y_max < x_max &&  i-2*x_max-y_max < y_max { 
            vec[i%n][y_min] = i as i32;
        }
        vec[0][i] = i as i32;
    }

    println!("{:#?}", vec);
    vec
}


#[cfg(test)]
mod tests {
    use super::generate_matrix;

   #[test]
    fn sample_test() {
        todo!()
    }
}
