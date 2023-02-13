/*!
 *
 * HackerLand University has the following grading policy:

Every student receives a  in the inclusive range from  to .
Any  less than  is a failing grade.
Sam is a professor at the university and likes to round each student's  according to these rules:

If the difference between the  and the next multiple of  is less than , round  up to the next multiple of .
If the value of  is less than , no rounding occurs as the result will still be a failing grade.
 */

/**
 * Complete the 'gradingStudents' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */
#[allow(unused)]
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    return grades.iter().map(|&grade| {
        if grade < 40 {
            return grade;
        } else {
            let next_mult_of_5 = (grade / 5 + 1) * 5;
            if next_mult_of_5 - grade < 3 {
                return next_mult_of_5;
            } else {
                return grade;
            }
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        assert_eq!(grading_students(&[73, 67, 38, 33]), [75, 67, 40, 33]);
        assert_eq!(grading_students(&[4, 73, 67, 38, 33]), [4, 75, 67, 40, 33]);
    }
}
