/*
For tuple, you can use many data type as you wish
For array, you can't

and also, by default, array is immutable
*/

#[test]
fn simple_array() {
    let arr: [u32; 5] = [1,2,3,4,5];
    println!("{arr:?}");

    let a = arr[0];
    let b = arr[1];
    println!("{a}, {b}");

    // in tuple you can't get length of tuple, but in array you can
    let length_of_arr: usize = arr.len();
    println!("{length_of_arr}");
}

#[test]
fn mutable_array() {
    let mut arr: [u32; 5] = [1,2,3,4,5];
    println!("{arr:?}");

    arr[0] = 12;
    arr[1] = arr[1] * 12;
    println!("{arr:?}");
}

#[test]
fn nth_dim_array() {
    let two_dim_matrix: [[i32; 2]; 2] = [
        [1,2],
        [3,4]
    ];
    println!("{two_dim_matrix:?}");

    let three_dim_matrix: [[[i32; 2]; 2]; 2] = [
        [[1,2],[3,4]],
        [[5,6],[7,8]]
    ];
    println!("{three_dim_matrix:?}");
}