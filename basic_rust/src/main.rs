// cargo new <project_name> to create a new rust project

// to add another file here, so it can be executed by compiler
// you must add it even for only simple unit test 
mod variable;
mod rust_type;
mod operations;
mod char_type;
mod tuple;
mod array;
mod memory_management;
mod string;

fn main() {
    // print!("Hello, world!"); // text
    println!("Hello, world!"); // text + "/n"
    // note: valid string is only "" not like php, js or python which can be '' or ""
    // note: we need to anotate ; (semicolon) here in every end of the line
    
    // to run project: cargo run
    // to build project: cargo build --release
    // then run with ./target/release/<project_name>
}

/*  
Rust can only create one main for one project, thus if we want to create another simple project
let say for learning, it requires to create multiple cargo new which is unecessary
so better to use unit test in here
*/

// example of unit test
#[test]
fn simple_test() {
    println!("Hello, world!. Test 123");
}

// to run this: cargo test <function_test_name> -- --exact --nocapture
// to run unittest outside main file: cargo test -- <filename (without .rs e.g.: var.rs -> var)>::<function_test_name> --exact --nocapture