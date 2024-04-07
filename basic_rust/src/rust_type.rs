/*
Rust divide type as 2 subset: scalar and compound
scalar: single value such as integer, float, boolean, char
compound: multiple value such as tuple, array
int: i8, i16, i32, i64, i128 (signed, - to +)
uint: u8, u16, u32, u64, u128 (unsigned, only +)
size: int type data based on platform, such as 32 bit cpu or 64 bit cpu
    isize: signed
    usize: unsigned
float: f32, f64

by default: rust use i32 for int and f64 for float
*/

// how to explicit type in rust
#[test]
fn explicit() {
    let debt: i8 = -20;
    println!("Total debt is: {} dollars", debt);
    let debt: i16 = -1_648;
    println!("Total debt is: {} dollars", debt);
    let debt: i32 = -1_648_984;
    println!("Total debt is: {} dollars", debt);

    let user: u8 = 20;
    println!("Active user is: {}", user);
    let user: u16 = 1_648;
    println!("Active user is: {}", user);
    let user: u32 = 1_648_984;
    println!("Active user is: {}", user);
}

// size conversion
#[test]
fn size_conv() {
    let a: i8 = 20;
    println!("8 bit int: {}", a);

    // notice this, this convert a to i16, without this, it will raise error
    // let b: i16 = a; // error
    let b: i16 = a as i16; //fine
    println!("16 bit int: {}", b);
}