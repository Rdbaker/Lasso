extern crate lasso;

use lasso::vector;

#[test]
fn chunk_n_larger_than_size() {
    assert_eq!(
        vector::chunk(vec![1,2,3], 4),
        vec![vec![1,2,3]]
    )
}

#[test]
fn chunk_leftover_small_chunk() {
    let v: Vec<Vec<i32>> = vector::chunk(vec![1,2,3], 2);
    assert_eq!(v, vec![vec![1, 2], vec![3]]);
}

#[test]
fn chunk_normal_testing() {
    let v: Vec<Vec<i32>> = vector::chunk(vec![1,2,3,4,5,6], 2);
    assert_eq!(v, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
}

#[test]
fn chunk_n_size_1() {
    let v: Vec<Vec<i32>> = vector::chunk(vec![1,2,3,4,5,6], 1);
    assert_eq!(v, vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]]);
}

#[test]
fn chunk_empty_vector() {
    assert_eq!(vec![vec![1]], vector::chunk(vec![1], 3));
}

#[test]
fn difference_full_first_vector() {
    assert_eq!(vec![1,2,3,4], vector::difference(vec![1,2,3,4], vec![]));
}

#[test]
fn difference_full_second_vector() {
    assert_eq!(vec![1,2,3,4], vector::difference(vec![], vec![1,2,3,4]));
}

#[test]
fn difference_no_overlap() {
    assert_eq!(vec![1,3,2,4], vector::difference(vec![1, 3], vec![2,4]));
}

#[test]
fn difference_partial_overlap() {
    assert_eq!(vec![1,4], vector::difference(vec![1, 2, 3], vec![2, 3, 4]));
}

#[test]
fn difference_no_diff() {
    let result: Vec<i32> = Vec::new();
    assert_eq!(result, vector::difference(vec![1, 2, 3, 4], vec![1, 2, 3, 4]));
}

#[test]
fn drop_n_0() {
    assert_eq!(
        vec![1, 2, 3, 4],
        vector::drop(vec![1, 2, 3, 4], 0)
    );
}

#[test]
fn drop_full_vec() {
    let result: Vec<i32> = Vec::new();
    assert_eq!(
        result,
        vector::drop(vec![1, 2, 3, 4], 4)
    );
}

#[test]
#[should_panic(expected = "out of range")]
fn drop_more_than_full_vec() {
    let result: Vec<i32> = Vec::new();
    assert_eq!(
        result,
        vector::drop(vec![1, 2, 3, 4], 5)
    );
}

#[test]
fn drop_partial_vec() {
    assert_eq!(
        vec![3, 4],
        vector::drop(vec![1, 2, 3, 4], 2)
    );
}
