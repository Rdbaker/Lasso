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


/// Creates a vector of values not included in the other given vector. The order and references of result values are determined by the first array.
///
/// # Examples
///
/// ```
/// use lasso::vector;
///
/// assert_eq!(
///     vector::difference(vec![1, 2, 3], vec![2, 3, 4]),
///     vec![1, 4]
/// )
/// ```
pub fn difference<T: Clone + PartialEq>(first_vec: Vec<T>, second_vec: Vec<T>) -> Vec<T> {
    let mut elts_unique_in_first: Vec<T> = first_vec.clone();
    let mut elts_unique_in_second: Vec<T> = second_vec.clone();

    elts_unique_in_first.retain(|ref elt| !second_vec.contains(&elt));
    elts_unique_in_second.retain(|ref elt| !first_vec.contains(&elt));

    elts_unique_in_first.append(&mut elts_unique_in_second);
    elts_unique_in_first
}


/// Creates a slice of the vector with n elements dropped from the beginning.
///
/// # Examples
///
/// ```
/// use lasso::vector;
///
/// assert_eq!(
///     vector::drop(vec![1, 2, 3], 2),
///     vec![3]
/// )
/// ```
pub fn drop<T: Clone>(vec: Vec<T>, n: usize) -> Vec<T> {
    let (_, vec_after_drop) = vec.split_at(n);
    vec_after_drop.to_vec()
}
