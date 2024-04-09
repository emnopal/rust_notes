/*
from: ownership_function.rs

Okay, so we already taken the ownership back, 
but it would be annoying if we must create tuple in return function again and again
it's okay if the params only have 2-3 params, what about 10 or even 100?

the answer is references

what is references?
references is basically pointer which pointing Heap
Data that point with reference still owned by another variable
so no transfer ownership occur here

if reference deleted or over (function terminate), the reference will deleted
but not with the data (since reference not owning the data, just reference to the pointer to data)

reference annotation ==> `&`
*/

#[cfg(test)]
fn compile_full_name3(first_name: &String, last_name: &String) -> String {
    // first_name = &String::from("Foo"); // cannot change value here, reference is immutable
    // if we set first_name to mut in compile_full_name3_test() it doesn't effect to here
    format!("{first_name} {last_name}")
}

#[test]
fn compile_full_name3_test() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let full_name = compile_full_name3(&first_name, &last_name);

    println!("{first_name}"); // not error anymore, since we reference value of first_name
    println!("{last_name}"); // idem
    println!("{full_name}");
}

/*
Reference is borrowing
and value of reference is immutable even though the owner is mutable
*/

// this will error

// #[cfg(test)]
// fn change_value(value: &String) {
//     value.push_str("Test"); // cannot borrow `*value` as mutable, as it is behind a `&` reference `value` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// }

// #[test]
// fn change_value_test() {
//     let mut value = String::from("Alexander");
//     change_value(&value);
//     println!("{value}");
// }

/*
But for some case, maybe we want to change the value
it can be done with &mut
*/

// the problem above solved
#[cfg(test)]
fn change_value(value: &mut String) {
    value.push_str(" Test");
}

#[test]
fn change_value_test() {
    let mut value = String::from("Alexander");
    change_value(&mut value);
    println!("{value}");
}

/*
Dangling Pointer

Dangling Pointer is the condition where reference (pointer) which point to value but it doesn't exists in memory

In rust (not like C/C++, Java, Go) it's prohibited
ex. when we return the value as reference, based on rust memory management, if the function done or terminated, 
the value automatically deleted since it was out of function scope

so if this happens, the rust will panic
*/

// panic

// #[cfg(test)]
// fn compile_full_name4(first_name: &String, last_name: &String) -> String {
//     let name = format!("{first_name} {last_name}");
//     return &name; // rust not allowing this
// }

// #[test]
// fn compile_full_name4_test() {
//     let first_name = String::from("Alexander");
//     let last_name = String::from("Bell");
//     let full_name = compile_full_name4(&first_name, &last_name); // will panic

//     println!("{first_name}"); // not error anymore, since we reference value of first_name
//     println!("{last_name}"); // idem
//     println!("{full_name}");
// }

// solution

#[cfg(test)]
fn compile_full_name4(first_name: &String, last_name: &String) -> String {
    let name = format!("{first_name} {last_name}");
    return name; // back to compile_full_name3
    // return name as value not as reference, so the ownership will moved to compile_full_name4_test
}

#[test]
fn compile_full_name4_test() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let full_name = compile_full_name4(&first_name, &last_name);

    println!("{first_name}");
    println!("{last_name}");
    println!("{full_name}");
}

// or

#[cfg(test)]
fn compile_full_name5(first_name: &String, last_name: &String, full_name: &mut String) -> () {
    full_name.push_str(&format!("{first_name} {last_name}"));
}

#[test]
fn compile_full_name5_test() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let mut full_name = String::new(); // Initialize an empty string
    compile_full_name5(&first_name, &last_name, &mut full_name);

    println!("{first_name}");
    println!("{last_name}");
    println!("{full_name}");
}