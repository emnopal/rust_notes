#[test]
fn simple_tuple() {
    // note: u have to explicitly write all tuple type inside tuple
    let data: (char, char, char) = ('a', 'b', 'c');
    // println!("{}", data); // will be error
    println!("{:?}", data); // you can use this
    println!("Pretty tuple: {:#?}", data); // or this to pretty print
    println!("{data:?} or {data:#?}"); // you can write like this

    // you can access tuple like this
    let access_a = data.0;
    let access_b = data.1;
    let access_c = data.2;
    println!("{}, {}, {}", access_a, access_b, access_c);

    // or using destructuring tuple like this
    let (a, b, c) = data;
    println!("{}, {}, {}", a, b, c);

    // or if you don't want to get c, you can do it like this
    let (a1, b1, _) = data;
    println!("{}, {}", a1, b1);
}

#[test]
fn mutable_tuple() {
    let mut data: (i32, i32, i32) = (2, 4, 6);
    println!("{:?}", data);

    data.0 = 12;
    data.1 = data.1 * 10;
    println!("{:?}", data);
}

/*
Unit: tuple without value (or return value), written as: ()
*/

// this always return () or unit
#[cfg(test)] // tell the compiler this is for test
fn void_func() {
    println!("Just it")
}

#[test]
fn unit_tuple() {
    // unit as return value
    let result = void_func();
    println!("{result:?}");

    // unit as default value
    let test = ();
    println!("{test:?}");
}