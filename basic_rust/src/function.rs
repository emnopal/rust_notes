#[cfg(test)]
fn simple_func() {
    println!("Test");
}

#[cfg(test)]
fn simple_func_with_return_type() -> () {
    println!("Test with return type");
}

#[test]
fn simple_func_test() {
    simple_func();
    simple_func_with_return_type();
}

#[cfg(test)]
fn func_with_params(first_name: &str, last_name: &str) {
    println!("Hello, {first_name} {last_name}");
}

#[test]
fn func_with_params_test() {
    let first_name = "Alexander";
    let last_name = "Bell";
    func_with_params(first_name, last_name);
    func_with_params("Graham", "Bell");
}

#[cfg(test)]
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result // if u want to return, don't use `;` at the end
    // return result; // or u can use return statement here with `;` at the end
}

#[test]
fn factorial_test() {
    let n: u64 = 5;
    println!("Factorial of {n}: {}", factorial(n))
}

#[cfg(test)]
fn factorial_recursion(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursion(n-1)
}

#[test]
fn factorial_recursion_test() {
    let n: u32 = 5;
    println!("Factorial of {n}: {}", factorial_recursion(n))
}

#[cfg(test)]
fn loop_with_recursion(text: String, times: u32) {
    if times == 0 {
        return;
    }
    println!("{times}. {text}");
    loop_with_recursion(text, times-1);
}

#[test]
fn loop_with_recursion_test() {
    let test_str = String::from("Test 123");
    loop_with_recursion(test_str, 20)
}