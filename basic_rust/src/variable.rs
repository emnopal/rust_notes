#[test]
fn simple() {
    // this is simple variable in rust
    let example_variable = "This is Rust Very Simple Variable";
    println!("Hello Guys, {}", example_variable);
}

#[test]
fn mutable() {
    // note: rust is static typing, e.g.: so str cannot be number, vice versa
    // by default, rust variable is immutable
    // we can override it with mut keyword on variable
    let mut example_variable = "This is Rust";
    println!("{}", example_variable);
    example_variable = "This is Rust Mutable Variable";
    println!("{}", example_variable);
}

#[test]
fn shadowing() {
    // shadowing is allowed in rust
    // but this is not best practice
    // pls consider first when u want to attemp this approach
    let var = "This is Rust";
    println!("{}", var);
    let var = 10;
    println!("{}", var);
}