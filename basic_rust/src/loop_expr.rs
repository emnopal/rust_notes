#[test]
fn loop_expr() {
    let mut counter = 0;

    // basically, this is while true
    loop {
        counter += 1;
        if counter >= 100 {
            break; // to break loop
        } else {
            continue; // to continue loop
        }
    }

    println!("{counter}")
}

#[test]
fn loop_expr_return() {
    let mut counter = 0;

    // basically, this is while true
    let result = loop { // basically it breaks then divide by 10
        counter += 1;
        if counter >= 100 {
            break counter / 10;
        } // or you can use if else or if else if else statement here
    };

    println!("{counter}");
    println!("{result}"); // should be 10
}

#[test]
fn loop_label() {
    // we can give a name to loop and break the loop with the name
    let mut number = 0;
    'outer: loop {
        let mut i = 0;
        'inner: loop {
            if number > 10 {
                break 'outer;
            }
            println!("{}x{}={}", number, i, number * i);
            i += 1;
            if i > 10 {
                break 'inner;
                // or simply just break
            }
        }
    number += 1;
    }
}

#[test]
fn while_loop() {
    let mut result: u32 = 0;
    let mut counter = 0;
    while counter <= 100 {
        result += counter;
        counter += 1;
    }
    println!("{result}");
}

#[test]
fn array_iter_while() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let mut index = 0;

    while index < arr.len() {
        println!("Index: {}, Value: {}", index, arr[index]);
        index += 1;
    }
}

#[test]
fn array_iter_for_index() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    for index in 0..arr.len() {
        println!("Index: {}, Value: {}", index, arr[index]);
    }
}

#[test]
fn array_iter_for_index2() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    for (index, value) in arr.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}

#[test]
fn array_iter_for_index3() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let range = 0..arr.len();
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for index in range {
        println!("Index: {}, Value: {}", index, arr[index]);
    }
}

#[test]
fn array_iter_for_inclusive_index4() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let range = 0..=4; // this is inclusive, equivalent with 0..5 or 0..(4+1) or 0..((5-1)+1)
    println!("Start: {}", range.start()); // notice diff here inclusive need call method, ordinary not
    println!("End: {}", range.end()); // notice diff here inclusive need call method, ordinary not

    for index in range {
        println!("Index: {}, Value: {}", index, arr[index]);
    }
}

#[test]
fn array_iter_for() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    for value in arr {
        println!("Value: {}", value);
    }
}

#[test]
fn iter_range() {
    for i in 0..10 {
        println!("Range: {i}");
    }
}

#[test]
fn iter_range_inclusive() {
    for i in 1..=10 { // this is inclusive, equivalent with 1..11 or 1..(10+1)
        println!("Range: {i}");
    }
}