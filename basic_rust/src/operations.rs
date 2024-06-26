#[test]
fn simple_numeric() {
    let a: i16 = 10;
    let b: i16 = 20;
    
    let mut c: i16 = a + b;
    println!("Add: {}", c);

    c = a - b;
    println!("Substract: {}", c);

    c = a * b;
    println!("Multiply: {}", c);

    c = a / b;
    println!("Divide: {}", c);

    c = a % b;
    println!("Modulo: {}", c);
}

#[test]
fn augmented() {
    let a: i16 = 10;
    let mut b: i16 = a;

    b += 10;
    println!("Add: {}", b);

    b -= 10;
    println!("Substract: {}", b);

    b *= 10;
    println!("Multiply: {}", b);

    b /= 10;
    println!("Divide: {}", b);

    b %= 10;
    println!("Modulo: {}", b);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{}, {}", a, b)
}

#[test]
fn comparison() {
    let a = 12;
    let b = 32;

    // comparison 1 (if else)
    if a != b {
        println!("a and b not equal")
    } else {
        println!("a and b equal")
    }

    // comparison 2 (if else if else)
    if a >= b {
        println!("a is greater or equal to b")
    } else if a > b {
        println!("a is greater than b")
    } else if a < b {
        println!("a is less than b")
    } else if a <= b {
        println!("a is less or equal to b")
    } else {
        println!("not match")
    }

    // comparison 3 (let statement)
    let result: &str;
    if a >= b {
        result = "a is greater or equal to b";
    } else if a > b {
        result = "a is greater than b";
    } else if a < b {
        result = "a is less than b";
    } else if a <= b {
        result = "a is less or equal to b";
    } else {
        result = "not match";
    }
    println!("{result}");

    // comparison 4 (if let statement)
    let res = if a >= b {
        "a is greater or equal to b"
    } else if a > b {
        "a is greater than b"
    } else if a < b {
        "a is less than b"
    } else if a <= b {
        "a is less or equal to b"
    } else {
        "not match"
    };
    println!("{res}");

    // comparison 5 (variable)
    let c: bool = a >= b;
    println!("does b is greater than equal to b: {c}");

    // comparison 6 (ternary)
    let d: &str = if a == b {"Equal"} else {"Not Equal"};
    println!("{d}");

    // comparison 7 (boolean ops)
    let f = 64;
    let g = f > 0 && f > a * b; // &&, it should be false since 12*32 >> 64
    println!("{g}");
    let h = f > 0 || f > a * b; // ||, it should be true since true || false
    println!("{h}");
    let i = !(f > 0 || f > a * b); // inverse (!), it should be false since inverse of (true || false)
    println!("{i}");
}