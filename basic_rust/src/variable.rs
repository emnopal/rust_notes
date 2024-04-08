#[test]
fn simple() {
    // this is simple variable in rust
    let example_variable = "This is Rust Very Simple Variable";
    println!("Hello Guys, {}", example_variable);
}

#[test]
fn simple_assign() {
    // or you can explicitly put variable inside string (f-string)
    let example_variable = "This is Rust Very Simple Variable with F-String";
    println!("Hello Guys, {example_variable}");
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

// note for constant, you have explisitly write their type
// global const
#[cfg(test)] // tell the compiler this is for test
const MAX_VALUE: i32 = 100;

#[test]
fn constant_var() {
    // local const
    const MIN_VALUE: i32 = -100;

    println!("{}, {}", MAX_VALUE, MIN_VALUE)
}

// scope
#[test]
fn scope() {
    let outer = 1;

    { // inner scope
        println!("Outer scope: {outer}");
        let inner = 2;
        println!("Inner scope: {inner}");
    }

    // println!("Inner scope: {inner}"); // sorry, u can't access here
}