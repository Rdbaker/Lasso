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
