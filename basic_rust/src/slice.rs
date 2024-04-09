/*
Slice in rust is reference
Slice basically reference of array (or vector)
*/

#[test]
fn slice_example() {
    let arr: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let slice1: &[i32] = &arr[..];
    println!("{slice1:?}");

    let slice2: &[i32] = &arr[0..5];
    println!("{slice2:?}");

    // or
    let slice3: &[i32] = &arr[..5];
    println!("{slice3:?}");

    // or
    let slice4: &[i32] = &arr[..=4];
    println!("{slice4:?}");

    // or
    let slice5: &[i32] = &arr[5..10];
    println!("{slice5:?}");

    // or
    let slice6: &[i32] = &arr[5..arr.len()];
    println!("{slice6:?}");

    // or
    let slice7: &[i32] = &arr[5..];
    println!("{slice7:?}");
}

/*
String Slice
&str is reference to str and str is slice
*/

#[test]
fn string_slice() {
    let name: String = String::from("Alexander Graham Bell");
    let first_name: &str = &name[..=8];
    println!("{first_name}");

    let other_name: &str = &name[10..];
    println!("{other_name}");
}

#[test]
fn str_slice() {
    // also can be applied with &str
    let name: &str = "Alexander Graham Bell";
    let first_name: &str = &name[..=8];
    println!("{first_name}");

    let other_name: &str = &name[10..];
    println!("{other_name}");
}