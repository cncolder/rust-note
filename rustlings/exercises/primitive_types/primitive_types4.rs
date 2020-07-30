// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// 数组切片前要先借值.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 从 1 到 3, 不包括 4.
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
