/*
Rust not use Garbage Collection (Python, Go, Javascript) nor Manual Memory Management (C/C++)
Rust divide data inside memory as 2 parts, Stack and Heap

Stack:
Stack is the data which stored using stacked structured data (last in first out)
All of the data must be fixed size

Heap:
Inside heap there is Memory Allocator which purpose is
to find empty space to store and allocate the data into that empty space, after it
the pointer will given into this data to point where exact the data is inside the heap
pointer which given by heap is fixed size, so the pointer will stored into stack

When variable out from their scope (which means it can't be accessed), automatically rust calling drop function
drop function is the function to delete the data from memory, which means it deleted from heap
and if rust function() was executed, so the function will deleted too from stack frame
so by that scheme, rust doesn't need neither garbage collection nor memory management
*/

#[cfg(test)]
fn function_a() {
    let a = 16;

    // notice length of b in function_a()
    // and function_b() diff? yes!, this is heap
    let b = String::from("Test"); // this is heap

    println!("{}, {}", a, b);
}

#[cfg(test)]
fn function_b() {
    let a = 10;

    // notice length of b in function_a()
    // and function_b() diff? yes!, this is heap
    let b = String::from("Test Function B"); // this is heap

    println!("{}, {}", a, b);
}

#[test]
fn stack_heap() {
    // this is stack
    function_a();
    function_b();
}