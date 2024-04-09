/*
Datatype whose stored in Heap such as String, etc
if we sent it as parameter in function
the ownership will moved to function that we called
and if the function executed, the parameter would be deleted

for datatype whose stored in Stack such as i32, &str, etc.
the params will copied, and won't be deleted when function executed
*/

// example of heap

#[cfg(test)]
fn heap_type_params(text: String) { // rustc will suggest that text params to borrow instead of owning
    println!("Printing, {text}")
}

#[test]
fn heap_type_params_test() {
    let text = String::from("Test Heap Params"); // rustc will suggest: better Copy this value
    heap_type_params(text); // value moved here
    // println!("{text}"); // this will be error, since `text` moving it's ownership into `fn heap_type_params`
}

// example of stack

#[cfg(test)]
fn stack_type_params(text: &str) {
    println!("Printing, {text}")
}

#[test]
fn stack_type_params_test() {
    let text = "Test Stack Params";
    stack_type_params(text); // value copied here
    println!("{text}"); // this will be fine
}

/*
Return value ownership

if we return Heap value in the function, automatically the ownership 
will moved to whose call the function

but, if the value is Stack, the return function will be copied to whose call the function
*/

// example of return value ownership in Heap

#[cfg(test)]
fn compile_full_name(first_name: String, last_name: String) -> String {
    format!("{first_name} {last_name}")
}

#[test]
fn compile_full_name_test() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let full_name = compile_full_name(first_name, last_name);

    // println!("{first_name}"); // error, since `compile_full_name()` doesn't return first_name and last_name, so the ownership doesn't moved again to it's first place
    // println!("{last_name}"); // also for this
    println!("{full_name}"); // instead, the compiler_full_name only return new string that format first_name and last_name which owned by variable full_name in here
}

/*
Since return like above doesn't returning the ownership,
how do we do to returning the ownership?

so, we can returning the ownership with tuple
*/

// returning ownership

#[cfg(test)]
fn compile_full_name2(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{first_name} {last_name}");
    (first_name, last_name, full_name)
}

#[test]
fn compile_full_name2_test() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let (first_name, last_name, full_name) = compile_full_name2(first_name, last_name); // don't forget to return tuple like this

    println!("{first_name}");
    println!("{last_name}");
    println!("{full_name}");
}

/*
Okay, so we already taken the ownership back, 
but it would be annoying if we must create tuple in return function again and again
it's okay if the params only have 2-3 params, what about 10 or even 100?

move explanation to: reference_borrowing.rs
*/