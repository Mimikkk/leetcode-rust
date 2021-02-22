use crate::utils::Form;
#[test]
fn sorted_should_return_new_sorted_vec() {
    use crate::utils::Sorted;
    let original_vector = vec![4, 3, 2, 1];

    assert_eq!(original_vector.sorted(), vec![1, 2, 3, 4], "Should return new sorted vec");
}
