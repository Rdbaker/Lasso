/// Split the vector into n-size chunks.
///
/// # Examples
///
/// ```
/// use lasso::vector;
///
/// assert_eq!(
///     vector::chunk(vec![1, 2, 3], 2),
///     vec![vec![1, 2], vec![3]]
/// )
/// ```
pub fn chunk<T: Clone>(vec: Vec<T>, n: usize) -> Vec<Vec<T>> {
    if n >= vec.len()  {
        vec![vec]
    } else {
        let mut outer_vec: Vec<Vec<T>> = vec![];
        let mut inner_vec: Vec<T> = vec![];

        for x in 0..vec.len() {
            if inner_vec.len() == n {
                let temp_vec = inner_vec.clone();
                outer_vec.push(temp_vec);
                inner_vec.clear();
            }
            inner_vec.push(vec.get(x).unwrap().clone());
        }

        if !inner_vec.is_empty() {
            let temp_vec = inner_vec.clone();
            outer_vec.push(temp_vec);
        }

        outer_vec
    }
}
