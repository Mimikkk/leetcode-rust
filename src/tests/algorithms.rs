use crate::algorithms::{pascal_triangle_i, pascal_triangle_ii};

#[test]
fn pascal_triangle_i_row_1_to_4() {
    assert_eq!(pascal_triangle_i(0), Vec::<Vec<i32>>::new(), "Should be []");
    assert_eq!(pascal_triangle_i(1), vec![vec![1]], "Should be [[1]]");
    assert_eq!(pascal_triangle_i(2), vec![vec![1], vec![1, 1]], "Should be [[1],[1,1]]");
    assert_eq!(pascal_triangle_i(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]], "Should be [[1],[1,1],[1,2,1]]");
    assert_eq!(pascal_triangle_i(4), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]], "Should be [[1],[1,1],[1,2,1],[1,3,3,1]]");
}

#[test]
fn pascal_triangle_ii_row_0_to_3() {
    assert_eq!(pascal_triangle_ii(0), vec![1], "Should be [1]");
    assert_eq!(pascal_triangle_ii(1), vec![1, 1], "Should be [1,1]");
    assert_eq!(pascal_triangle_ii(2), vec![1, 2, 1], "Should be [1,2,1]");
    assert_eq!(pascal_triangle_ii(3), vec![1, 3, 3, 1], "Should be [1,3,3,1]");
}
